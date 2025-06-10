use super::generate_map_infos_data;
use super::generate_ruby_data;
use super::set_extension_str;
use super::FileSink;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use rpgmvx_ace_types::Script;
use rpgmvx_ace_types::ScriptList;
use ruby_marshal::IntoValue;
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;

fn generate_scripts_data_vx_ace(path: &Path) -> anyhow::Result<Vec<u8>> {
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

pub fn compile(
    entry_path: &Path,
    entry_file_type: std::fs::FileType,
    relative_path: &Path,
    relative_path_components: Vec<&str>,
    file_sink: &mut FileSink,
) -> anyhow::Result<()> {
    match relative_path_components.as_slice() {
        ["Data", "Scripts.rvdata2"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let scripts_data = generate_scripts_data_vx_ace(entry_path)?;
            let size = u32::try_from(scripts_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*scripts_data)?;
        }
        ["Data", "Scripts.rvdata2", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "MapInfos.rvdata2"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let data = generate_map_infos_data(entry_path)?;
            let size = u32::try_from(data.len())?;

            file_sink.write_file(&relative_path_components, size, &*data)?;
        }
        ["Data", "MapInfos.rvdata2", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", file] if crate::util::is_map_file_name(file, "json") => {
            println!("packing \"{}\"", relative_path.display());

            let map_data = generate_ruby_data::<rpgmvx_ace_types::Map>(entry_path)?;
            let size = u32::try_from(map_data.len())?;

            let renamed_file = set_extension_str(file, "rvdata2");
            let mut relative_path_components = relative_path_components.clone();
            *relative_path_components.last_mut().unwrap() = renamed_file.as_str();

            file_sink.write_file(&relative_path_components, size, &*map_data)?;
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

    Ok(())
}
