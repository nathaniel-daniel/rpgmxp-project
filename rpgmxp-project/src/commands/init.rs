use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use flate2::bufread::ZlibDecoder;
use std::collections::HashSet;
use std::fmt::Write;
use std::io::Read;
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
    let data_path = options.from.join("Data");
    let graphics_path = options.from.join("Graphics");

    ensure!(
        data_path.exists(),
        "missing folder at \"{}\"",
        data_path.display()
    );
    let out_dir = options.output.join("Data");
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
                let scripts_data = std::fs::read(in_path)?;
                let value_arena = ruby_marshal::load(&*scripts_data)?;
                let mut visited_values = HashSet::new();

                let out_dir = out_dir.join("Scripts");
                std::fs::create_dir_all(&out_dir)?;

                let script_list = match value_arena
                    .get(value_arena.root())
                    .context("invalid handle")?
                {
                    ruby_marshal::Value::Array(value) => value,
                    _ => bail!("script list was not an array"),
                };

                for (script_index, handle) in script_list.value().iter().enumerate() {
                    let script = match value_arena.get(*handle).context("invalid handle")? {
                        ruby_marshal::Value::Array(value) => value.value(),
                        _ => bail!("script was not an array"),
                    };
                    ensure!(script.len() == 3);

                    let _script_id: i32 = ruby_marshal::FromValue::from_value(
                        &value_arena,
                        script[0],
                        &mut visited_values,
                    )?;
                    let script_name: &ruby_marshal::StringValue =
                        ruby_marshal::FromValue::from_value(
                            &value_arena,
                            script[1],
                            &mut visited_values,
                        )?;
                    let script_name = std::str::from_utf8(script_name.value())?;
                    let script_data: &ruby_marshal::StringValue =
                        ruby_marshal::FromValue::from_value(
                            &value_arena,
                            script[2],
                            &mut visited_values,
                        )?;
                    let mut decoder = ZlibDecoder::new(script_data.value());
                    let mut script_data = String::new();
                    decoder.read_to_string(&mut script_data)?;

                    let mut escaped_script_name = String::with_capacity(script_name.len());
                    for c in script_name.chars() {
                        match c {
                            '%' | ':' => {
                                let c = u32::from(c);
                                write!(&mut escaped_script_name, "%{c:x}")?;
                            }
                            _ => {
                                escaped_script_name.push(c);
                            }
                        }
                    }

                    let out_path = out_dir.join(format!("{script_index}-{escaped_script_name}.rb"));
                    std::fs::write(&out_path, script_data)?;
                }
            }
            _ => {}
        }
    }

    // todo!();

    ensure!(
        graphics_path.exists(),
        "missing folder at \"{}\"",
        graphics_path.display()
    );
    let out_dir = options.output.join("Graphics");
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
