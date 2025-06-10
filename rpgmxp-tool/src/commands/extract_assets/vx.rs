use super::extract_map_infos;
use super::extract_ruby_data;
use super::extract_scripts;
use super::FileEntry;
use super::Options;
use anyhow::Context;
use std::fs::File;
use std::path::PathBuf;

pub fn extract(
    options: &Options,
    entry: &mut FileEntry<'_>,
    relative_path_components: Vec<&str>,
    output_path: PathBuf,
) -> anyhow::Result<()> {
    match relative_path_components.as_slice() {
        ["Data", "Scripts.rvdata"] if !options.skip_extract_scripts => {
            extract_scripts(entry, output_path)?;
        }
        ["Data", "MapInfos.rvdata"] if !options.skip_extract_map_infos => {
            extract_map_infos(entry, output_path)?;
        }
        ["Data", "System.rvdata"] if !options.skip_extract_system => {
            extract_ruby_data::<rpgmvx_types::System>(entry, output_path)?;
        }
        ["Data", file]
            if !options.skip_extract_maps && crate::util::is_map_file_name(file, "rvdata") =>
        {
            extract_ruby_data::<rpgmvx_types::Map>(entry, output_path)?;
        }
        _ => {
            let temp_path = nd_util::with_push_extension(&output_path, "temp");
            // TODO: Lock?
            // TODO: Drop delete guard for file?
            let mut output_file = File::create(&temp_path)
                .with_context(|| format!("failed to open file at \"{}\"", output_path.display()))?;

            std::io::copy(entry, &mut output_file)?;
            std::fs::rename(&temp_path, &output_path)?;
        }
    }

    Ok(())
}
