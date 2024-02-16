mod map;
mod scripts;

use self::map::Map;
use self::scripts::ScriptList;
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

        let file_stem = in_path
            .file_stem()
            .context("missing file stem")?
            .to_str()
            .context("file stem is not valid unicode")?;

        let is_map = file_stem.strip_prefix("Map").map_or(false, |file_stem| {
            file_stem.len() == 3 && file_stem.chars().all(|c| c.is_ascii_digit())
        });

        if is_map {
            //if file_stem <= "Map001" {
            //    continue;
            //}

            let map_data = std::fs::read(in_path)?;
            let value_arena = ruby_marshal::load(&*map_data)?;
            let mut visited_values = HashSet::new();

            let maybe_map: Result<Map, _> = ruby_marshal::FromValue::from_value(
                &value_arena,
                value_arena.root(),
                &mut visited_values,
            );

            if let Err(ruby_marshal::FromValueError::Cycle { handle }) = maybe_map.as_ref() {
                dbg!(handle);
                dbg!(value_arena.get(*handle));
            }

            let map = maybe_map?;

            dbg!(map);

            continue;
        }

        // We will add more later.
        #[allow(clippy::single_match)]
        match file_stem {
            "Scripts" => {
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

    let script_list: ScriptList =
        ruby_marshal::FromValue::from_value(&value_arena, value_arena.root(), &mut visited_values)?;

    for (script_index, script) in script_list.scripts.iter().enumerate() {
        let escaped_script_name = escape_file_name(&script.name);

        let out_path = out_dir.join(format!("{script_index}-{escaped_script_name}.rb"));
        std::fs::write(&out_path, &script.data)?;
    }

    Ok(())
}

fn escape_file_name(file_name: &str) -> String {
    let mut escaped = String::with_capacity(file_name.len());
    for c in file_name.chars() {
        match c {
            '%' | ':' => {
                let c = u32::from(c);
                write!(&mut escaped, "%{c:02x}").unwrap();
            }
            _ => {
                escaped.push(c);
            }
        }
    }
    escaped
}
