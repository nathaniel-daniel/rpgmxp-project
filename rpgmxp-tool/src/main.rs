mod commands;
mod util;

use anyhow::bail;
use std::str::FromStr;

/// The game kind
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameKind {
    Xp,
    Vx,
    VxAce,
}

impl GameKind {
    /// Returns true if this is xp.
    pub fn is_xp(self) -> bool {
        matches!(self, Self::Xp)
    }

    /// Returns true if this is vx.
    pub fn is_vx(self) -> bool {
        matches!(self, Self::Vx)
    }

    /// Returns true if this is vx ace.
    pub fn is_vx_ace(self) -> bool {
        matches!(self, Self::VxAce)
    }
}

impl FromStr for GameKind {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.eq_ignore_ascii_case("xp") || input.eq_ignore_ascii_case("rgssad") {
            return Ok(Self::Xp);
        }

        if input.eq_ignore_ascii_case("vx") || input.eq_ignore_ascii_case("rgss2a") {
            return Ok(Self::Vx);
        }

        if input.eq_ignore_ascii_case("vx-ace")
            || input.eq_ignore_ascii_case("rgss3a")
            || input.eq_ignore_ascii_case("ace")
        {
            return Ok(Self::Vx);
        }

        bail!("\"{input}\" is not a valid game kind");
    }
}

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
