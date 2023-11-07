mod commands;

#[derive(Debug, argh::FromArgs)]
#[argh(description = "a CLI to build RPGMaker XP assets")]
struct Options {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, argh::FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Init(self::commands::init::Options),
}

fn main() -> anyhow::Result<()> {
    let options: Options = argh::from_env();

    match options.subcommand {
        Subcommand::Init(options) => self::commands::init::exec(options)?,
    }

    Ok(())
}
