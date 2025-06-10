use super::extract_map_infos;
use super::extract_ruby_data;
use super::FileEntry;
use super::Options;
use anyhow::Context;
use ruby_marshal::FromValueContext;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

fn extract_scripts_vx_ace<P>(file: impl std::io::Read, dir_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    let temp_dir_path = nd_util::with_push_extension(dir_path, "temp");

    // TODO: Lock?
    // TODO: Drop delete guard for file?
    std::fs::create_dir_all(&temp_dir_path)?;

    let arena = ruby_marshal::load(file).context("failed to load ruby data")?;
    let ctx = FromValueContext::new(&arena);
    let script_list: rpgmvx_ace_types::ScriptList = ctx.from_value(arena.root())?;

    for (script_index, script) in script_list.scripts.iter().enumerate() {
        println!("  extracting script \"{}\"", script.name);

        let escaped_script_name = crate::util::percent_escape_file_name(&script.name);

        let out_path = temp_dir_path.join(format!("{script_index:03}-{escaped_script_name}.rb"));
        let temp_path = nd_util::with_push_extension(&out_path, "temp");

        // TODO: Lock?
        // TODO: Drop delete guard for file?
        std::fs::write(&temp_path, &script.data)?;
        std::fs::rename(temp_path, out_path)?;
    }

    std::fs::rename(temp_dir_path, dir_path)?;

    Ok(())
}

pub fn extract(
    options: &Options,
    entry: &mut FileEntry<'_>,
    relative_path_components: Vec<&str>,
    output_path: PathBuf,
) -> anyhow::Result<()> {
    match relative_path_components.as_slice() {
        ["Data", "Scripts.rvdata2"] if !options.skip_extract_scripts => {
            extract_scripts_vx_ace(entry, output_path)?;
        }
        ["Data", "MapInfos.rvdata2"] if !options.skip_extract_map_infos => {
            extract_map_infos(entry, output_path)?;
        }
        ["Data", file]
            if !options.skip_extract_maps && crate::util::is_map_file_name(file, "rvdata2") =>
        {
            extract_ruby_data::<rpgmvx_ace_types::Map>(entry, output_path)?;
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
