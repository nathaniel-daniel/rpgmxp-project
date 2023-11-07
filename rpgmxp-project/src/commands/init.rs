mod scripts;

use self::scripts::Script;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use std::collections::HashSet;
use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, argh::FromArgs)]
#[argh(name = "init", subcommand, description = "init a new project")]
pub struct Options {
    #[argh(
        option,
        description = "the path to the game or archive to init a project from"
    )]
    pub from: PathBuf,

    #[argh(option, description = "the output path", default = "PathBuf::new()")]
    pub output: PathBuf,
}

pub fn exec(options: Options) -> anyhow::Result<()> {
    copy_data(&options.from, &options.output)?;
    copy_graphics(&options.from, &options.output)?;
    Ok(())
}

fn copy_data(base_in_path: &Path, base_out_path: &Path) -> anyhow::Result<()> {
    let data_path = base_in_path.join("Data");
    ensure!(
        data_path.exists(),
        "missing folder at \"{}\"",
        data_path.display()
    );
    let out_dir = base_out_path.join("Data");
    for entry in std::fs::read_dir(data_path)? {
        let entry = entry?;

        let file_type = entry.file_type()?;
        ensure!(file_type.is_file());

        let in_path = entry.path();
        ensure!(in_path.extension() == Some("rxdata".as_ref()));

        let file_stem = in_path.file_stem().context("missing file stem")?;
        // We will add more later.
        #[allow(clippy::single_match)]
        match file_stem.to_str() {
            Some("Scripts") => {
                let out_dir = out_dir.join("Scripts");
                std::fs::create_dir_all(&out_dir)?;
                extract_scripts(&in_path, &out_dir)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn copy_graphics(base_in_path: &Path, base_out_path: &Path) -> anyhow::Result<()> {
    let graphics_path = base_in_path.join("Graphics");

    ensure!(
        graphics_path.exists(),
        "missing folder at \"{}\"",
        graphics_path.display()
    );
    let out_dir = base_out_path.join("Graphics");
    for entry in std::fs::read_dir(graphics_path)? {
        let entry = entry?;

        // TODO: Should we allow files in non-standard places?
        let file_type = entry.file_type()?;
        ensure!(file_type.is_dir());

        let dir_name = entry.file_name();
        let out_dir = out_dir.join(dir_name);

        std::fs::create_dir_all(&out_dir)?;

        for entry in std::fs::read_dir(entry.path())? {
            let entry = entry?;

            let file_type = entry.file_type()?;
            ensure!(file_type.is_file());

            let in_path = entry.path();
            let out_path = out_dir.join(entry.file_name());
            std::fs::copy(&in_path, &out_path).with_context(|| {
                format!(
                    "failed to copy \"{}\" to \"{}\"",
                    in_path.display(),
                    out_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn extract_scripts(in_path: &Path, out_dir: &Path) -> anyhow::Result<()> {
    let scripts_data = std::fs::read(in_path)?;
    let value_arena = ruby_marshal::load(&*scripts_data)?;
    let mut visited_values = HashSet::new();

    let script_list = match value_arena
        .get(value_arena.root())
        .context("invalid handle")?
    {
        ruby_marshal::Value::Array(value) => value,
        _ => bail!("script list was not an array"),
    };

    for (script_index, handle) in script_list.value().iter().enumerate() {
        let script: Script =
            ruby_marshal::FromValue::from_value(&value_arena, *handle, &mut visited_values)?;

        let escaped_script_name = escape_file_name(&script.name);

        let out_path = out_dir.join(format!("{script_index}-{escaped_script_name}.rb"));
        std::fs::write(&out_path, script.data)?;
    }

    Ok(())
}

fn escape_file_name(file_name: &str) -> String {
    let mut escaped = String::with_capacity(file_name.len());
    for c in file_name.chars() {
        match c {
            '%' | ':' => {
                let c = u32::from(c);
                write!(&mut escaped, "%{c:x}").unwrap();
            }
            _ => {
                escaped.push(c);
            }
        }
    }
    escaped
}
