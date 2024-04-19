mod file_entry_iter;

use self::file_entry_iter::FileEntryIter;
use anyhow::ensure;
use anyhow::Context;
use camino::Utf8Path;
use rpgmxp_types::Map;
use rpgmxp_types::ScriptList;
use ruby_marshal::FromValueContext;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, argh::FromArgs)]
#[argh(
    subcommand,
    name = "unpack",
    description = "unpack a game into a format that is modifiable"
)]
pub struct Options {
    #[argh(
        positional,
        description = "the path to the game folder or rgssad archive"
    )]
    pub input: PathBuf,

    #[argh(positional, description = "the folder to unpack to")]
    pub output: PathBuf,

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

    ensure!(!options.output.exists(), "output path exists");

    // TODO: Should we validate the output in some way?
    // Prevent writing to the input dir?
    std::fs::create_dir_all(&options.output)?;

    options.output = options
        .output
        .canonicalize()
        .context("failed to canonicalize output path")?;

    let mut file_entry_iter = FileEntryIter::new(&options.input)?;

    while let Some(mut entry) = file_entry_iter.next_file_entry()? {
        let raw_relative_path = entry.relative_path();
        let relative_path_components = parse_relative_path(raw_relative_path)?;
        let relative_path_display = relative_path_components.join("/");
        let output_path = {
            let mut output_path = options.output.clone();
            output_path.extend(relative_path_components.clone());
            output_path
        };

        eprintln!("extracting \"{relative_path_display}\"");

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create dir at \"{}\"", parent.display()))?;
        }

        match relative_path_components.as_slice() {
            ["Data", "Scripts.rxdata"] if !options.skip_extract_scripts => {
                extract_scripts(entry, output_path)?;
            }
            ["Data", "CommonEvents.rxdata"] if !options.skip_extract_common_events => {
                extract_common_events(entry, output_path)?;
            }
            ["Data", "System.rxdata"] if !options.skip_extract_system => {
                extract_system(entry, output_path)?;
            }
            ["Data", "Actors.rxdata"] if !options.skip_extract_actors => {
                extract_actors(entry, output_path)?;
            }
            ["Data", file]
                if !options.skip_extract_maps && crate::util::is_map_file_name(file, "rxdata") =>
            {
                extract_map(entry, output_path)?;
            }
            _ => {
                let temp_path = nd_util::with_push_extension(&output_path, "temp");
                // TODO: Lock?
                // TODO: Drop delete guard for file?
                let mut output_file = File::create(&temp_path).with_context(|| {
                    format!("failed to open file at \"{}\"", output_path.display())
                })?;

                std::io::copy(&mut entry, &mut output_file)?;
                std::fs::rename(&temp_path, &output_path)?;
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

fn extract_scripts<P>(file: impl std::io::Read, dir_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    let temp_dir_path = nd_util::with_push_extension(dir_path, "temp");

    // TODO: Lock?
    // TODO: Drop delete guard for file?
    std::fs::create_dir_all(&temp_dir_path)?;

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let script_list: ScriptList = ctx.from_value(arena.root())?;

    for (script_index, script) in script_list.scripts.iter().enumerate() {
        println!("  extracting script \"{}\"", script.name);

        let escaped_script_name = crate::util::percent_escape_file_name(&script.name);

        let out_path = temp_dir_path.join(format!("{script_index}-{escaped_script_name}.rb"));
        let temp_path = nd_util::with_push_extension(&out_path, "temp");

        // TODO: Lock?
        // TODO: Drop delete guard for file?
        std::fs::write(&temp_path, &script.data)?;
        std::fs::rename(temp_path, out_path)?;
    }

    std::fs::rename(temp_dir_path, dir_path)?;

    Ok(())
}

fn extract_common_events<P>(file: impl std::io::Read, dir_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    let temp_dir_path = nd_util::with_push_extension(dir_path, "temp");

    // TODO: Lock?
    // TODO: Drop delete guard for file?
    std::fs::create_dir_all(&temp_dir_path)?;

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let common_events: Vec<Option<rpgmxp_types::CommonEvent>> = ctx.from_value(arena.root())?;

    for (common_event_index, common_event) in common_events.iter().enumerate() {
        if common_event_index == 0 {
            ensure!(common_event.is_none(), "common event 0 should be nil");

            continue;
        }

        let common_event = common_event.as_ref().context("common event is nil")?;

        println!("  extracting common event \"{}\"", common_event.name);

        let common_event_name = common_event.name.as_str();
        let out_path = temp_dir_path.join(format!("{common_event_index}-{common_event_name}.json"));
        let temp_path = nd_util::with_push_extension(&out_path, "temp");

        // TODO: Lock?
        // TODO: Drop delete guard for file?
        let mut output_file = File::create_new(&temp_path)?;
        serde_json::to_writer_pretty(&mut output_file, common_event)?;
        output_file.flush()?;
        output_file.sync_all()?;
        drop(output_file);

        std::fs::rename(temp_path, out_path)?;
    }

    std::fs::rename(temp_dir_path, dir_path)?;

    Ok(())
}

fn extract_system<P>(file: impl std::io::Read, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let path = path.with_extension("json");

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let system: rpgmxp_types::System = ctx.from_value(arena.root())?;

    let temp_path = nd_util::with_push_extension(&path, "temp");
    let mut file = File::create_new(&temp_path)?;
    serde_json::to_writer_pretty(&mut file, &system)?;
    file.flush()?;
    file.sync_all()?;
    std::fs::rename(temp_path, path)?;

    Ok(())
}

fn extract_actors<P>(file: impl std::io::Read, dir_path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    let temp_dir_path = nd_util::with_push_extension(dir_path, "temp");

    // TODO: Lock?
    // TODO: Drop delete guard for file?
    std::fs::create_dir_all(&temp_dir_path)?;

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let actors: Vec<Option<rpgmxp_types::Actor>> = ctx.from_value(arena.root())?;

    for (actor_index, actor) in actors.iter().enumerate() {
        if actor_index == 0 {
            ensure!(actor.is_none(), "actor 0 should be nil");

            continue;
        }

        let actor = actor.as_ref().context("actor is nil")?;

        println!("  extracting actor \"{}\"", actor.name);

        let actor_name = actor.name.as_str();
        let out_path = temp_dir_path.join(format!("{actor_index}-{actor_name}.json"));
        let temp_path = nd_util::with_push_extension(&out_path, "temp");

        // TODO: Lock?
        // TODO: Drop delete guard for file?
        let mut output_file = File::create_new(&temp_path)?;
        serde_json::to_writer_pretty(&mut output_file, actor)?;
        output_file.flush()?;
        output_file.sync_all()?;
        drop(output_file);

        std::fs::rename(temp_path, out_path)?;
    }

    std::fs::rename(temp_dir_path, dir_path)?;

    Ok(())
}

fn extract_map<P>(file: impl std::io::Read, path: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let path = path.with_extension("json");

    let arena = ruby_marshal::load(file)?;
    let ctx = FromValueContext::new(&arena);
    let map: Map = ctx.from_value(arena.root())?;
    let map = serde_json::to_string_pretty(&map)?;

    // TODO: Lock?
    // TODO: Drop delete guard for file?
    let temp_path = nd_util::with_push_extension(&path, "temp");
    std::fs::write(&temp_path, map)?;

    std::fs::rename(temp_path, path)?;

    Ok(())
}
