mod file_sink;

use self::file_sink::FileSink;
use crate::util::ArrayLikeElement;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use rpgmxp_types::Actor;
use rpgmxp_types::Armor;
use rpgmxp_types::Class;
use rpgmxp_types::CommonEvent;
use rpgmxp_types::Enemy;
use rpgmxp_types::Item;
use rpgmxp_types::MapInfo;
use rpgmxp_types::Script;
use rpgmxp_types::ScriptList;
use rpgmxp_types::Skill;
use rpgmxp_types::State;
use rpgmxp_types::Tileset;
use rpgmxp_types::Troop;
use rpgmxp_types::Weapon;
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

    #[argh(
        switch,
        long = "overwrite",
        description = "whether overwrite the output if it exists"
    )]
    pub overwrite: bool,
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
        Format::Dir => FileSink::new_dir(&options.output, options.overwrite)?,
        Format::Rgssad => FileSink::new_rgssad(&options.output, options.overwrite)?,
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

                let rx_data = generate_arraylike_rx_data::<CommonEvent>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "CommonEvents.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Actors.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Actor>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Actors.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Weapons.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Weapon>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Weapons.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Armors.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Armor>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Armors.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Skills.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Skill>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Skills.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "States.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<State>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "States.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Items.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Item>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Items.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Enemies.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Enemy>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Enemies.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Classes.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Class>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Classes.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Troops.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Troop>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Troops.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "Tilesets.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_arraylike_rx_data::<Tileset>(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "Tilesets.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "MapInfos.rxdata"] if entry_file_type.is_dir() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_map_infos_rx_data(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
            }
            ["Data", "MapInfos.rxdata", ..] => {
                // Ignore entries, we explore them in the above branch.
            }
            ["Data", "System.json"] if entry_file_type.is_file() => {
                println!("packing \"{}\"", relative_path.display());

                let rx_data = generate_system_rx_data(entry_path)?;
                let size = u32::try_from(rx_data.len())?;

                let mut relative_path_components = relative_path_components.clone();
                *relative_path_components.last_mut().unwrap() = "System.rxdata";

                file_sink.write_file(&relative_path_components, size, &*rx_data)?;
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

fn generate_arraylike_rx_data<T>(path: &Path) -> anyhow::Result<Vec<u8>>
where
    T: for<'a> ArrayLikeElement<'a>,
{
    fn load_json_str(
        dir_entry: std::io::Result<std::fs::DirEntry>,
        type_display_name: &str,
    ) -> anyhow::Result<(usize, String)> {
        let dir_entry = dir_entry?;
        let dir_entry_file_type = dir_entry.file_type()?;

        ensure!(dir_entry_file_type.is_file());

        let dir_entry_file_name = dir_entry.file_name();
        let dir_entry_file_name = dir_entry_file_name.to_str().context("non-unicode name")?;
        let dir_entry_file_stem = dir_entry_file_name
            .strip_suffix(".json")
            .context("not a \"json\" file")?;

        let (index, name) = dir_entry_file_stem
            .split_once('-')
            .context("invalid name format")?;
        let name = crate::util::percent_unescape_file_name(name)?;
        let index: usize = index.parse()?;

        println!("  packing {type_display_name} \"{name}\"");

        let dir_entry_path = dir_entry.path();
        let json = std::fs::read_to_string(dir_entry_path)?;

        Ok((index, json))
    }

    let type_display_name = T::type_display_name();
    let mut map: BTreeMap<usize, T> = BTreeMap::new();

    for dir_entry in path.read_dir()? {
        let (index, json) = load_json_str(dir_entry, type_display_name)?;
        let value: T = serde_json::from_str(&json)?;

        let old_entry = map.insert(index, value);
        if old_entry.is_some() {
            bail!("duplicate {type_display_name} for index {index}");
        }
    }

    // TODO: Consider enforcing that value index ranges cannot have holes and must start at 1.
    let mut data = Vec::with_capacity(map.len() + 1);
    data.push(None);
    for value in map.into_values() {
        data.push(Some(value));
    }

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = data.into_value(&mut arena)?;
    arena.replace_root(handle);

    let mut data = Vec::new();
    ruby_marshal::dump(&mut data, &arena)?;

    Ok(data)
}

fn generate_map_infos_rx_data(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut map: BTreeMap<i32, MapInfo> = BTreeMap::new();

    for dir_entry in path.read_dir()? {
        let dir_entry = dir_entry?;
        let dir_entry_file_type = dir_entry.file_type()?;

        ensure!(dir_entry_file_type.is_file());

        let dir_entry_file_name = dir_entry.file_name();
        let dir_entry_file_name = dir_entry_file_name.to_str().context("non-unicode name")?;
        let dir_entry_file_stem = dir_entry_file_name
            .strip_suffix(".json")
            .context("not a \"json\" file")?;

        let (index, name) = dir_entry_file_stem
            .split_once('-')
            .context("invalid name format")?;
        let index: i32 = index.parse()?;

        println!("  packing map info \"{name}\"");

        let dir_entry_path = dir_entry.path();
        let json = std::fs::read_to_string(dir_entry_path)?;

        let value: MapInfo = serde_json::from_str(&json)?;

        let old_entry = map.insert(index, value);
        if old_entry.is_some() {
            bail!("duplicate map info for index {index}");
        }
    }

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = map.into_value(&mut arena)?;
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
