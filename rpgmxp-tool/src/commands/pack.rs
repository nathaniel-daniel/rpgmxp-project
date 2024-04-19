mod file_sink;

use self::file_sink::FileSink;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use rpgmxp_types::CommonEvent;
use rpgmxp_types::Script;
use rpgmxp_types::ScriptList;
use ruby_marshal::IntoValue;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Component as PathComponent;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

#[derive(Debug)]
enum Format {
    Dir,
    Rgssad,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dir" => Ok(Self::Dir),
            "rgssad" => Ok(Self::Rgssad),
            _ => bail!("unknown format \"{input}\""),
        }
    }
}

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "pack",
    description = "pack a folder unpacked with the unpack command"
)]
pub struct Options {
    #[argh(positional, description = "the input folder path to pack")]
    input: PathBuf,

    #[argh(positional, description = "the output path")]
    output: PathBuf,

    #[argh(
        option,
        long = "format",
        short = 'f',
        description = "the output format. Defaults to \"rgssad\" if the extension is for an rgssad file. Otherwise, \"dir\" is used."
    )]
    format: Option<Format>,
}

pub fn exec(mut options: Options) -> anyhow::Result<()> {
    options.input = options
        .input
        .canonicalize()
        .context("failed to canonicalize input path")?;

    let format = match options.format {
        Some(format) => format,
        None => {
            let extension = options
                .output
                .extension()
                .map(|extension| extension.to_str().context("non-unicode extension"))
                .transpose()?;
            if extension == Some("rgssad") || extension == Some("rgss2a") {
                Format::Rgssad
            } else {
                Format::Dir
            }
        }
    };

    let mut file_sink = match format {
        Format::Dir => FileSink::new_dir(&options.output)?,
        Format::Rgssad => FileSink::new_rgssad(&options.output)?,
    };

    for entry in WalkDir::new(&options.input) {
        let entry = entry?;
        let entry_file_type = entry.file_type();
        let entry_path = entry.path();

        let relative_path = entry_path.strip_prefix(&options.input)?;
        let relative_path_components = relative_path
            .components()
            .map(|component| match component {
                PathComponent::Normal(value) => value.to_str().context("non-unicode path"),
                component => bail!("unexpected path component \"{component:?}\""),
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        match relative_path_components.as_slice() {
            ["Data", "Scripts.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let scripts_rx_data = generate_scripts_rx_data(entry_path)?;
                let size = u32::try_from(scripts_rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*scripts_rx_data)?;
            }
            ["Data", "Scripts.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "CommonEvents.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let common_events_rx_data = generate_common_events_rx_data(entry_path)?;
                let size = u32::try_from(common_events_rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*common_events_rx_data)?;
            }
            ["Data", "CommonEvents.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "System.json"] if entry_file_type.is_file() => {
                println!("packing \"{}\"", relative_path.display());

                let system_rx_data = generate_system_rx_data(entry_path)?;
                let size = u32::try_from(system_rx_data.len())?;

                let mut relative_path_components = relative_path_components.clone();
                *relative_path_components.last_mut().unwrap() = "System.rxdata";

                file_sink.write_file(&relative_path_components, size, &*system_rx_data)?;
            }
            ["Data", file] if crate::util::is_map_file_name(file, "json") => {
                println!("packing \"{}\"", relative_path.display());

                let map_rx_data = generate_map_rx_data(entry_path)?;
                let size = u32::try_from(map_rx_data.len())?;

                let renamed_file = set_extension_str(file, "rxdata");
                let mut relative_path_components = relative_path_components.clone();
                *relative_path_components.last_mut().unwrap() = renamed_file.as_str();

                file_sink.write_file(&relative_path_components, size, &*map_rx_data)?;
            }
            relative_path_components if entry_file_type.is_file() => {
                // Copy file by default
                println!("packing \"{}\"", relative_path.display());

                let input_file = File::open(entry_path).with_context(|| {
                    format!(
                        "failed to open input file from \"{}\"",
                        entry_path.display()
                    )
                })?;
                let metadata = input_file.metadata()?;
                let size = u32::try_from(metadata.len())?;

                file_sink.write_file(relative_path_components, size, input_file)?;
            }
            _ => {}
        }
    }

    file_sink.finish()?;

    Ok(())
}

fn set_extension_str(input: &str, extension: &str) -> String {
    let stem = input
        .rsplit_once('.')
        .map(|(stem, _extension)| stem)
        .unwrap_or(input);

    format!("{stem}.{extension}")
}

fn generate_scripts_rx_data(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut scripts_map = BTreeMap::new();

    for dir_entry in path.read_dir()? {
        let dir_entry = dir_entry?;
        let dir_entry_file_type = dir_entry.file_type()?;

        ensure!(dir_entry_file_type.is_file());

        let dir_entry_file_name = dir_entry.file_name();
        let dir_entry_file_name = dir_entry_file_name
            .to_str()
            .context("non-unicode script name")?;
        let dir_entry_file_stem = dir_entry_file_name
            .strip_suffix(".rb")
            .context("script is not an \"rb\" file")?;

        let (script_index, escaped_script_name) = dir_entry_file_stem
            .split_once('-')
            .context("invalid script name format")?;
        let script_index: usize = script_index.parse()?;
        let unescaped_file_name = crate::util::percent_unescape_file_name(escaped_script_name)?;

        println!("  packing script \"{escaped_script_name}\"");

        let dir_entry_path = dir_entry.path();
        let script_data = std::fs::read_to_string(dir_entry_path)?;

        let old_entry = scripts_map.insert(
            script_index,
            Script {
                data: script_data,
                id: i32::try_from(script_index)? + 1,
                name: unescaped_file_name,
            },
        );
        if old_entry.is_some() {
            bail!("duplicate scripts for index {script_index}");
        }
    }

    // TODO: Consider enforcing that script index ranges cannot have holes and must start at 0.
    let script_list = ScriptList {
        scripts: scripts_map.into_values().collect(),
    };

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = script_list.into_value(&mut arena)?;
    arena.replace_root(handle);

    let mut data = Vec::new();
    ruby_marshal::dump(&mut data, &arena)?;

    Ok(data)
}

fn generate_common_events_rx_data(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut common_events_map: BTreeMap<usize, CommonEvent> = BTreeMap::new();

    for dir_entry in path.read_dir()? {
        let dir_entry = dir_entry?;
        let dir_entry_file_type = dir_entry.file_type()?;

        ensure!(dir_entry_file_type.is_file());

        let dir_entry_file_name = dir_entry.file_name();
        let dir_entry_file_name = dir_entry_file_name
            .to_str()
            .context("non-unicode script name")?;
        let dir_entry_file_stem = dir_entry_file_name
            .strip_suffix(".json")
            .context("common event is not a \"json\" file")?;

        let (common_event_index, common_event_name) = dir_entry_file_stem
            .split_once('-')
            .context("invalid common event name format")?;
        let common_event_index: usize = common_event_index.parse()?;

        println!("  packing script \"{common_event_name}\"");

        let dir_entry_path = dir_entry.path();
        let common_event_json = std::fs::read_to_string(dir_entry_path)?;
        let common_event: CommonEvent = serde_json::from_str(&common_event_json)?;

        let old_entry = common_events_map.insert(common_event_index, common_event);
        if old_entry.is_some() {
            bail!("duplicate common events for index {common_event_index}");
        }
    }

    // TODO: Consider enforcing that common event index ranges cannot have holes and must start at 1.
    let mut common_events = Vec::with_capacity(common_events_map.len() + 1);
    common_events.push(None);
    for common_event in common_events_map.into_values() {
        common_events.push(Some(common_event));
    }

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = common_events.into_value(&mut arena)?;
    arena.replace_root(handle);

    let mut data = Vec::new();
    ruby_marshal::dump(&mut data, &arena)?;

    Ok(data)
}

fn generate_system_rx_data(path: &Path) -> anyhow::Result<Vec<u8>> {
    let system = std::fs::read_to_string(path)?;
    let system: rpgmxp_types::System = serde_json::from_str(&system)?;

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = system.into_value(&mut arena)?;
    arena.replace_root(handle);

    let mut data = Vec::new();
    ruby_marshal::dump(&mut data, &arena)?;

    Ok(data)
}

fn generate_map_rx_data(path: &Path) -> anyhow::Result<Vec<u8>> {
    let map = std::fs::read_to_string(path)?;
    let map: rpgmxp_types::Map = serde_json::from_str(&map)?;

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = map.into_value(&mut arena)?;
    arena.replace_root(handle);

    let mut data = Vec::new();
    ruby_marshal::dump(&mut data, &arena)?;

    Ok(data)
}
