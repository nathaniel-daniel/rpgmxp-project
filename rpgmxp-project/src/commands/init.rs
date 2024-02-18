use anyhow::ensure;
use anyhow::Context;
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

        let map_number = file_stem.strip_prefix("Map").and_then(|file_stem| {
            if file_stem.len() != 3 {
                return None;
            }

            if !file_stem.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }

            Some(file_stem)
        });

        if let Some(map_number) = map_number {
            //if file_stem <= "Map001" {
            //    continue;
            //}

            let map_data = std::fs::read(&in_path)?;
            let value_arena = ruby_marshal::load(&*map_data)?;
            let ctx = ruby_marshal::FromValueContext::new(&value_arena);

            let maybe_map: Result<rpgmxp_types::Map, _> = ctx.from_value(value_arena.root());

            if let Err(ruby_marshal::FromValueError::UnexpectedValueKind { kind, trace }) =
                maybe_map.as_ref()
            {
                dbg!(kind);
                for handle in trace.iter().copied() {
                    let value = value_arena.get(handle).unwrap();
                    dbg!(DebugValue::new(&value_arena, value, 10));
                }
            }

            let map = maybe_map
                .with_context(|| format!("failed to extract data from Map{map_number:03}"))?;

            let out_path = out_dir.join(format!("Map{map_number}.json"));
            std::fs::write(&out_path, &serde_json::to_string_pretty(&map)?)?;

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
    let ctx = ruby_marshal::FromValueContext::new(&value_arena);

    let script_list: rpgmxp_types::ScriptList = ctx.from_value(value_arena.root())?;

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

struct DebugValue<'a> {
    arena: &'a ruby_marshal::ValueArena,
    value: &'a ruby_marshal::Value,
    limit: usize,
}

impl<'a> DebugValue<'a> {
    fn new(
        arena: &'a ruby_marshal::ValueArena,
        value: &'a ruby_marshal::Value,
        limit: usize,
    ) -> Self {
        Self {
            arena,
            value,
            limit,
        }
    }
}

impl std::fmt::Debug for DebugValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.limit == 0 {
            return self.value.fmt(f);
        }

        match &self.value {
            ruby_marshal::Value::Bool(value) => value.value().fmt(f),
            ruby_marshal::Value::Fixnum(value) => value.value().fmt(f),
            ruby_marshal::Value::String(value) => {
                let value = value.value();
                match std::str::from_utf8(value) {
                    Ok(value) => value.fmt(f),
                    Err(_error) => value.fmt(f),
                }
            }
            ruby_marshal::Value::Array(value) => {
                let mut f = f.debug_list();
                for handle in value.value().iter().copied() {
                    match self.arena.get(handle) {
                        Some(value) => {
                            f.entry(&DebugValue::new(self.arena, value, self.limit - 1));
                        }
                        None => {
                            f.entry(&handle);
                        }
                    }
                }
                f.finish()
            }
            ruby_marshal::Value::Object(value) => {
                let name = value.name();
                let name = match self
                    .arena
                    .get_symbol(name)
                    .and_then(|value| std::str::from_utf8(value.value()).ok())
                {
                    Some(name) => name,
                    None => {
                        return value.fmt(f);
                    }
                };

                let instance_variables = value.instance_variables();

                let mut f = f.debug_struct(name);

                for (key, value) in instance_variables.iter().copied() {
                    let key = self.arena.get_symbol(key).unwrap().value();
                    let key = std::str::from_utf8(key).unwrap();

                    let value = self.arena.get(value).unwrap();

                    f.field(key, &DebugValue::new(self.arena, value, self.limit - 1));
                }

                f.finish()
            }
            _ => self.value.fmt(f),
        }
    }
}
