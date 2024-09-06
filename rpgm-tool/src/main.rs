mod commands;
mod util;

#[derive(Debug, argh::FromArgs)]
#[argh(description = "a cli tool to interact with rpgmaker xp and vx games")]
struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    ExtractAssets(self::commands::extract_assets::Options),
    CompileAssets(self::commands::compile_assets::Options),
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();
    match options.subcommand {
        Subcommand::ExtractAssets(options) => self::commands::extract_assets::exec(options)?,
        Subcommand::CompileAssets(options) => self::commands::compile_assets::exec(options)?,
    }

    Ok(())
}
