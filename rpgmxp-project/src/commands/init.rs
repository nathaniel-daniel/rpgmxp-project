use anyhow::ensure;
use anyhow::Context;
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

    ensure!(
        graphics_path.exists(),
        "missing folder at \"{}\"",
        graphics_path.display()
    );
    for entry in std::fs::read_dir(graphics_path)? {
        let entry = entry?;

        // TODO: Should we allow files in non-standard places?
        let file_type = entry.file_type()?;
        ensure!(file_type.is_dir());

        let dir_name = entry.file_name();
        let out_dir = options.output.join(dir_name);

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
