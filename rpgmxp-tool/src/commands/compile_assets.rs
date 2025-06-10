mod file_sink;
mod vx;
mod vx_ace;
mod xp;

use self::file_sink::FileSink;
use crate::util::ArrayLikeElement;
use crate::GameKind;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use rpgm_common_types::Script;
use rpgm_common_types::ScriptList;
use ruby_marshal::IntoValue;
use std::collections::BTreeMap;
use std::path::Component as PathComponent;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use walkdir::WalkDir;

fn set_extension_str(input: &str, extension: &str) -> String {
    let stem = input
        .rsplit_once('.')
        .map(|(stem, _extension)| stem)
        .unwrap_or(input);

    format!("{stem}.{extension}")
}

#[derive(Debug)]
enum Format {
    Dir,
    Rgssad,
    Rgss2a,
    Rgss3a,
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "dir" => Ok(Self::Dir),
            "rgssad" => Ok(Self::Rgssad),
            "rgss2a" => Ok(Self::Rgss2a),
            "rgss3a" => Ok(Self::Rgss3a),
            _ => bail!("unknown format \"{input}\""),
        }
    }
}

fn generate_ruby_data<T>(path: &Path) -> anyhow::Result<Vec<u8>>
where
    T: serde::de::DeserializeOwned + ruby_marshal::IntoValue,
{
    let map = std::fs::read_to_string(path)?;
    let map: T = serde_json::from_str(&map)?;

    let mut arena = ruby_marshal::ValueArena::new();
    let handle = map.into_value(&mut arena)?;
    arena.replace_root(handle);

    let mut data = Vec::new();
    ruby_marshal::dump(&mut data, &arena)?;

    Ok(data)
}

fn generate_scripts_data(path: &Path) -> anyhow::Result<Vec<u8>> {
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

fn generate_map_infos_data(path: &Path) -> anyhow::Result<Vec<u8>> {
    let mut map: BTreeMap<i32, rpgm_common_types::MapInfo> = BTreeMap::new();

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

        let value: rpgm_common_types::MapInfo = serde_json::from_str(&json)?;

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

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "compile-assets",
    description = "recompile extracted assets from a folder"
)]
pub struct Options {
    #[argh(positional, description = "the input folder path to compile")]
    input: PathBuf,

    #[argh(positional, description = "the output path")]
    output: PathBuf,

    #[argh(
        option,
        long = "format",
        short = 'f',
        description = "the output format. Defaults to detecting from the extension. Otherwise, \"dir\" is used."
    )]
    format: Option<Format>,

    #[argh(
        option,
        long = "game",
        short = 'g',
        description = "the game type. Defaults to detecting from the output format. Must be provided if the output format is a dir."
    )]
    game: Option<GameKind>,

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
            if extension == Some("rgssad") {
                Format::Rgssad
            } else if extension == Some("rgss2a") {
                Format::Rgss2a
            } else if extension == Some("rgss3a") {
                Format::Rgss3a
            } else {
                Format::Dir
            }
        }
    };

    let mut file_sink = match format {
        Format::Dir => FileSink::new_dir(&options.output, options.overwrite)?,
        Format::Rgssad | Format::Rgss2a | Format::Rgss3a => {
            FileSink::new_rgssad(&options.output, options.overwrite)?
        }
    };
    let game_kind = options.game.map(Ok).unwrap_or_else(|| match format {
        Format::Dir => {
            bail!("need to provide game type with --game flag when outputting to a dir.")
        }
        Format::Rgssad => Ok(GameKind::Xp),
        Format::Rgss2a => Ok(GameKind::Vx),
        Format::Rgss3a => Ok(GameKind::VxAce),
    })?;

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

        match game_kind {
            GameKind::Xp => self::xp::compile(
                entry_path,
                entry_file_type,
                relative_path,
                relative_path_components,
                &mut file_sink,
            )?,
            GameKind::Vx => self::vx::compile(
                entry_path,
                entry_file_type,
                relative_path,
                relative_path_components,
                &mut file_sink,
            )?,
            GameKind::VxAce => self::vx_ace::compile(
                entry_path,
                entry_file_type,
                relative_path,
                relative_path_components,
                &mut file_sink,
            )?,
        }
    }

    file_sink.finish()?;

    Ok(())
}
