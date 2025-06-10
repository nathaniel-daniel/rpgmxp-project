use super::generate_map_infos_data;
use super::generate_ruby_data;
use super::generate_scripts_data;
use super::set_extension_str;
use super::FileSink;
use anyhow::Context;
use std::fs::File;
use std::path::Path;

pub fn compile(
    entry_path: &Path,
    entry_file_type: std::fs::FileType,
    relative_path: &Path,
    relative_path_components: Vec<&str>,
    file_sink: &mut FileSink,
) -> anyhow::Result<()> {
    match relative_path_components.as_slice() {
        ["Data", "Scripts.rvdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let scripts_data = generate_scripts_data(entry_path)?;
            let size = u32::try_from(scripts_data.len())?;

            file_sink.write_file(&relative_path_components, size, &*scripts_data)?;
        }
        ["Data", "Scripts.rvdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "MapInfos.rvdata"] if entry_file_type.is_dir() => {
            println!("packing \"{}\"", relative_path.display());

            let data = generate_map_infos_data(entry_path)?;
            let size = u32::try_from(data.len())?;

            file_sink.write_file(&relative_path_components, size, &*data)?;
        }
        ["Data", "MapInfos.rvdata", ..] => {
            // Ignore entries, we explore them in the above branch.
        }
        ["Data", "System.json"] if entry_file_type.is_file() => {
            println!("packing \"{}\"", relative_path.display());

            let data = generate_ruby_data::<rpgmvx_types::System>(entry_path)?;
            let size = u32::try_from(data.len())?;

            let mut relative_path_components = relative_path_components.clone();
            *relative_path_components.last_mut().unwrap() = "System.rvdata";

            file_sink.write_file(&relative_path_components, size, &*data)?;
        }
        ["Data", file] if crate::util::is_map_file_name(file, "json") => {
            println!("packing \"{}\"", relative_path.display());

            let map_data = generate_ruby_data::<rpgmvx_types::Map>(entry_path)?;
            let size = u32::try_from(map_data.len())?;

            let renamed_file = set_extension_str(file, "rvdata");
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
