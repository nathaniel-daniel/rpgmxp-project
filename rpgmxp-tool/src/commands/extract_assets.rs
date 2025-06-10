mod file_entry_iter;
mod vx;
mod vx_ace;
mod xp;

use self::file_entry_iter::FileEntry;
use self::file_entry_iter::FileEntryIter;
use crate::util::ArrayLikeElement;
use crate::GameKind;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use camino::Utf8Path;
use ruby_marshal::FromValueContext;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn extract_ruby_data<T>(file: impl std::io::Read, path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: serde::Serialize + for<'a> ruby_marshal::FromValue<'a>,
{
    let path = path.as_ref();
    let path = path.with_extension("json");

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let data: T = ctx.from_value(arena.root())?;

    // TODO: Lock?
    // TODO: Drop delete guard for file?
    let temp_path = nd_util::with_push_extension(&path, "temp");
    let mut file = File::create_new(&temp_path)?;
    serde_json::to_writer_pretty(&mut file, &data)?;
    file.flush()?;
    file.sync_all()?;
    std::fs::rename(temp_path, path)?;

    Ok(())
}

fn extract_arraylike<T>(file: impl std::io::Read, dir_path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: for<'a> ArrayLikeElement<'a>,
{
    let dir_path = dir_path.as_ref();
    let type_display_name = T::type_display_name();

    std::fs::create_dir_all(dir_path)?;

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let array: Vec<Option<T>> = ctx.from_value(arena.root())?;

    for (index, value) in array.iter().enumerate() {
        if index == 0 {
            ensure!(value.is_none(), "{type_display_name} 0 should be nil");
            continue;
        }

        let value = value
            .as_ref()
            .with_context(|| format!("{type_display_name} is nil"))?;

        println!("  extracting {} \"{}\"", type_display_name, value.name());

        let name = value.name();
        let file_name = format!("{index:03}-{name}.json");
        let file_name = crate::util::percent_escape_file_name(file_name.as_str());
        let out_path = dir_path.join(file_name);
        let temp_path = nd_util::with_push_extension(&out_path, "temp");

        // TODO: Lock?
        // TODO: Drop delete guard for file?
        let mut output_file = File::create_new(&temp_path)?;
        serde_json::to_writer_pretty(&mut output_file, value)?;
        output_file.flush()?;
        output_file.sync_all()?;
        drop(output_file);

        std::fs::rename(temp_path, out_path)?;
    }

    Ok(())
}

fn extract_scripts<P>(file: impl std::io::Read, dir_path: P) -> anyhow::Result<()>
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
    let script_list: rpgm_common_types::ScriptList = ctx.from_value(arena.root())?;

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

fn extract_map_infos<P>(file: impl std::io::Read, dir_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();

    std::fs::create_dir_all(dir_path)?;

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let map: BTreeMap<i32, rpgm_common_types::MapInfo> = ctx.from_value(arena.root())?;

    for (index, value) in map.iter() {
        let name = value.name.as_str();

        println!("  extracting map info \"{name}\"");

        let out_path = dir_path.join(format!("{index:03}-{name}.json"));
        let temp_path = nd_util::with_push_extension(&out_path, "temp");

        // TODO: Lock?
        // TODO: Drop delete guard for file?
        let mut output_file = File::create_new(&temp_path)?;
        serde_json::to_writer_pretty(&mut output_file, value)?;
        output_file.flush()?;
        output_file.sync_all()?;
        drop(output_file);

        std::fs::rename(temp_path, out_path)?;
    }

    Ok(())
}

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "extract-assets",
    description = "extract the assets from a game into a format that is modifiable"
)]
pub struct Options {
    #[argh(
        positional,
        description = "the path to the game folder or rgssad archive"
    )]
    pub input: PathBuf,

    #[argh(positional, description = "the folder to extract-assets to")]
    pub output: PathBuf,

    #[argh(
        switch,
        long = "overwrite",
        description = "whether overwrite the output directory"
    )]
    pub overwrite: bool,

    #[argh(
        switch,
        long = "skip-extract-scripts",
        description = "whether scripts should not be extracted"
    )]
    pub skip_extract_scripts: bool,

    #[argh(
        switch,
        long = "skip-extract-common-events",
        description = "whether common events should not be extracted"
    )]
    pub skip_extract_common_events: bool,

    #[argh(
        switch,
        long = "skip-extract-system",
        description = "whether system data should not be extracted"
    )]
    pub skip_extract_system: bool,

    #[argh(
        switch,
        long = "skip-extract-actors",
        description = "whether actors should not be extracted"
    )]
    pub skip_extract_actors: bool,

    #[argh(
        switch,
        long = "skip-extract-weapons",
        description = "whether weapons should not be extracted"
    )]
    pub skip_extract_weapons: bool,

    #[argh(
        switch,
        long = "skip-extract-armors",
        description = "whether armor should not be extracted"
    )]
    pub skip_extract_armors: bool,

    #[argh(
        switch,
        long = "skip-extract-skills",
        description = "whether skills should not be extracted"
    )]
    pub skip_extract_skills: bool,

    #[argh(
        switch,
        long = "skip-extract-states",
        description = "whether states should not be extracted"
    )]
    pub skip_extract_states: bool,

    #[argh(
        switch,
        long = "skip-extract-items",
        description = "whether items should not be extracted"
    )]
    pub skip_extract_items: bool,

    #[argh(
        switch,
        long = "skip-extract-enemies",
        description = "whether enemies should not be extracted"
    )]
    pub skip_extract_enemies: bool,

    #[argh(
        switch,
        long = "skip-extract-classes",
        description = "whether classes should not be extracted"
    )]
    pub skip_extract_classes: bool,

    #[argh(
        switch,
        long = "skip-extract-troops",
        description = "whether troops should not be extracted"
    )]
    pub skip_extract_troops: bool,

    #[argh(
        switch,
        long = "skip-extract-tilesets",
        description = "whether tilesets should not be extracted"
    )]
    pub skip_extract_tilesets: bool,

    #[argh(
        switch,
        long = "skip-extract-map-infos",
        description = "whether map infos should not be extracted"
    )]
    pub skip_extract_map_infos: bool,

    #[argh(
        switch,
        long = "skip-extract-animations",
        description = "whether animations should not be extracted"
    )]
    pub skip_extract_animations: bool,

    #[argh(
        switch,
        long = "skip-extract-maps",
        description = "whether maps should not be extracted"
    )]
    pub skip_extract_maps: bool,
}

pub fn exec(mut options: Options) -> anyhow::Result<()> {
    options.input = options
        .input
        .canonicalize()
        .context("failed to canonicalize input path")?;

    if options.output.try_exists()? {
        if options.overwrite {
            std::fs::remove_dir_all(&options.output)?;
        } else {
            bail!("output path exists");
        }
    }

    // TODO: Should we validate the output in some way?
    // Prevent writing to the input dir?
    std::fs::create_dir_all(&options.output)?;

    options.output = options
        .output
        .canonicalize()
        .context("failed to canonicalize output path")?;

    let mut file_entry_iter = FileEntryIter::new(&options.input)?;
    let game_kind = file_entry_iter.game_kind();

    while let Some(mut entry) = file_entry_iter.next_file_entry()? {
        let raw_relative_path = entry.relative_path().to_path_buf();
        let relative_path_components = parse_relative_path(&raw_relative_path)?;
        let relative_path_display = relative_path_components.join("/");
        let output_path = {
            let mut output_path = options.output.clone();
            output_path.extend(relative_path_components.clone());
            output_path
        };

        println!("extracting \"{relative_path_display}\"");

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create dir at \"{}\"", parent.display()))?;
        }

        match game_kind {
            GameKind::Xp => {
                self::xp::extract(&options, &mut entry, relative_path_components, output_path)?
            }
            GameKind::Vx => {
                self::vx::extract(&options, &mut entry, relative_path_components, output_path)?
            }
            GameKind::VxAce => {
                self::vx_ace::extract(&options, &mut entry, relative_path_components, output_path)?
            }
        }
    }

    Ok(())
}

fn parse_relative_path(path: &Utf8Path) -> anyhow::Result<Vec<&str>> {
    let mut components = Vec::with_capacity(4);

    // There is a lot of problems with using proper path parsing here.
    //
    // Since we need to accept both Windows and Unix paths here on any host,
    // since we may be running on Unix and rgssad archives use Windows paths.
    // This means we cannot use std's paths.
    //
    // We need to ensure that the given paths do not have hostile components,
    // like .. or C:. This is because rgssad archives are user supplied.
    // This means we still need to parse the paths somehow.
    //
    // The `typed-paths` crate seems ideal superficially,
    // but path conversions can easily cause paths with prefixes to replace the root.
    //
    // As a result, we need to do our own parsing here and be as conservative as possible.

    for component in path.as_str().split(['/', '\\']) {
        ensure!(!component.is_empty());
        ensure!(component != "..");
        ensure!(!component.contains(':'));

        if component == "." {
            continue;
        }

        components.push(component);
    }

    Ok(components)
}
