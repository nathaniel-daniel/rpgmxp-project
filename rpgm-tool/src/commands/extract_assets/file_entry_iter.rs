use crate::GameKind;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(serde::Deserialize, Debug)]
pub struct Assembly {
    #[serde(rename = "assemblyIdentity")]
    pub assembly_identity: AssemblyIdentity,

    pub description: Option<Description>,
}

#[derive(serde::Deserialize, Debug)]
#[expect(dead_code)]
pub struct AssemblyIdentity {
    #[serde(rename = "@version")]
    pub version: String,

    #[serde(rename = "@processorArchitecture")]
    pub processor_architecture: Option<String>,

    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@type")]
    pub type_: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Description {
    #[serde(rename = "$value")]
    pub value: String,
}

/// See: https://learn.microsoft.com/en-us/windows/win32/menurc/resource-types
/// See: https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/6c085406-a698-4e12-9d4d-c3b0ee3dbc4a
fn guess_game_kind_from_exe(game_exe: &[u8]) -> anyhow::Result<Option<GameKind>> {
    use object::pe::RT_MANIFEST;
    use object::read::File;
    use object::LittleEndian as LE;

    let file = File::parse(game_exe)?;
    let (section_table, data_directories) = match file {
        File::Pe32(file) => (file.section_table(), file.data_directories()),
        File::Pe64(file) => (file.section_table(), file.data_directories()),
        _ => bail!("unknown object file format {:?}", file.format()),
    };

    let resource_directory = data_directories.resource_directory(game_exe, &section_table)?;
    let resource_directory = match resource_directory {
        Some(resource_directory) => resource_directory,
        None => return Ok(None),
    };

    let root = resource_directory.root()?;
    let manifest_entry = root
        .entries
        .iter()
        .find(|entry| entry.name_or_id().id() == Some(RT_MANIFEST));
    let manifest_entry = match manifest_entry {
        Some(manifest_entry) => manifest_entry,
        None => return Ok(None),
    };

    let manifest_entry_data = manifest_entry.data(resource_directory)?;
    let manifest_entry_table = manifest_entry_data
        .table()
        .context("object MANIFEST data is not a table")?;

    let manifest_entry_table_entry_data = manifest_entry_table
        .entries
        .first()
        .context("object MANIFEST table missing entry 0")?
        .data(resource_directory)?;
    let manifest_entry_table_entry_data_table = manifest_entry_table_entry_data
        .table()
        .context("object MANIFEST table entry 0 is not a table")?;

    let manifest_entry_table_entry_data_table_entry_data = manifest_entry_table_entry_data_table
        .entries
        .first()
        .context("object MANIFEST table entry 0 table missing entry 0")?
        .data(resource_directory)?
        .data()
        .context("object MANIFEST table entry 0 table entry 0 is not data")?;
    let manifest_offset = manifest_entry_table_entry_data_table_entry_data
        .offset_to_data
        .get(LE);
    let manifest_size = usize::try_from(
        manifest_entry_table_entry_data_table_entry_data
            .size
            .get(LE),
    )?;
    let manifest_code_page = manifest_entry_table_entry_data_table_entry_data
        .code_page
        .get(LE);

    let manifest_bytes = &section_table
        .pe_data_at(game_exe, manifest_offset)
        .context("failed to get object manifest bytes")?
        .get(..manifest_size)
        .context("object manifest smaller than declared")?;

    let manifest_string = match manifest_code_page {
        0 => {
            // This isn't a real LCID from what I can tell,
            // but rather a null value. Assume ASCII for now.
            // TODO: Detect encoding dynamically?

            std::str::from_utf8(manifest_bytes)?.to_string()
        }
        _ => bail!("unknown manifest LCID {manifest_code_page}"),
    };

    let manifest: Assembly = quick_xml::de::from_str(&manifest_string)?;
    if manifest.assembly_identity.name == "Enterbrain.RGSS.Game"
        && manifest
            .description
            .as_ref()
            .map(|description| description.value.as_str())
            == Some("RGSS Player")
    {
        return Ok(Some(GameKind::Xp));
    }

    Ok(None)
}

/// A lending iter over files.
pub enum FileEntryIter {
    WalkDir {
        input_path: PathBuf,
        iter: walkdir::IntoIter,
        game_kind: GameKind,
    },
    Rgssad {
        reader: rgssad::Reader<File>,
        game_kind: GameKind,
    },
}

impl FileEntryIter {
    /// Create a new iter from a path.
    ///
    /// This will determine whether the path is a dir or an rgssad.
    pub fn new<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        if !path.is_dir() {
            // TODO: Add option to change rgssad version instead of assuming v1.
            return Self::new_rgssad_path(path);
        }

        let rgssad_path = path.join("Game.rgssad");
        match File::open(&rgssad_path) {
            Ok(file) => {
                return Self::new_rgssad_file(file, GameKind::Xp);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to open \"{}\"", rgssad_path.display()));
            }
        };

        let rgssad_path = path.join("Game.rgss2a");
        match File::open(&rgssad_path) {
            Ok(file) => {
                return Self::new_rgssad_file(file, GameKind::Vx);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to open \"{}\"", rgssad_path.display()));
            }
        };

        ensure!(
            path.join("Data").exists(),
            "Data directory is missing. Are you sure the input folder is correct?"
        );
        ensure!(
            path.join("Graphics").exists(),
            "Graphics directory is missing. Are you sure the input folder is correct?"
        );

        Self::new_walkdir_path(path)
    }

    /// Create a new iter from the given dir path.
    pub fn new_walkdir_path<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        let game_kind = (|| {
            let game_exe = std::fs::read(path.join("Game.exe"))?;
            if memchr::memmem::find(&game_exe, b"R\x00G\x00S\x00S\x002\x00").is_some() {
                return Ok(GameKind::Vx);
            }

            if let Some(game_kind) = guess_game_kind_from_exe(&game_exe)? {
                return Ok(game_kind);
            }

            bail!("failed to determine game type");
        })()?;

        let iter = WalkDir::new(path).into_iter();

        Ok(FileEntryIter::WalkDir {
            input_path: path.into(),
            iter,
            game_kind,
        })
    }

    /// Create a new iter from the given rgssad path.
    pub fn new_rgssad_path<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let extension = path
            .extension()
            .context("missing extension")?
            .to_str()
            .context("extension is not unicode")?;
        let game_kind: GameKind = extension.parse()?;
        let file = File::open(path)
            .with_context(|| format!("failed to open input file from \"{}\"", path.display()))?;
        Self::new_rgssad_file(file, game_kind)
    }

    /// Create a new iter from the given rgssad file.
    pub fn new_rgssad_file(file: File, game_kind: GameKind) -> anyhow::Result<Self> {
        let mut reader = rgssad::Reader::new(file);
        reader.read_header()?;

        Ok(Self::Rgssad { reader, game_kind })
    }

    /// Get the next file entry.
    pub fn next_file_entry(&mut self) -> anyhow::Result<Option<FileEntry>> {
        match self {
            Self::WalkDir {
                input_path, iter, ..
            } => {
                // Filter out dir entries, to keep similar behavior with rgssad.
                let entry = loop {
                    match iter.next() {
                        Some(Ok(entry)) if !entry.file_type().is_dir() => break entry,
                        Some(Ok(_entry)) => {}
                        Some(Err(error)) => return Err(error).context("failed to read dir entry"),
                        None => return Ok(None),
                    };
                };
                ensure!(!entry.path_is_symlink());

                let file = File::open(entry.path())?;

                let entry_path = entry.into_path();
                let relative_path = entry_path.strip_prefix(input_path)?;
                let relative_path = relative_path
                    .to_str()
                    .context("relative path is not utf8")?;

                Ok(Some(FileEntry::WalkDir {
                    relative_path: relative_path.into(),
                    file,
                }))
            }
            Self::Rgssad { reader, .. } => {
                let file = match reader.read_file()? {
                    Some(file) => file,
                    None => return Ok(None),
                };

                Ok(Some(FileEntry::Rgssad { file }))
            }
        }
    }

    /// Get the determined game kind
    pub fn game_kind(&self) -> GameKind {
        match self {
            Self::WalkDir { game_kind, .. } => *game_kind,
            Self::Rgssad { game_kind, .. } => *game_kind,
        }
    }
}

/// A file entry
pub enum FileEntry<'a> {
    WalkDir {
        relative_path: Utf8PathBuf,
        file: File,
    },
    Rgssad {
        file: rgssad::reader::File<'a, File>,
    },
}

impl FileEntry<'_> {
    /// Get the relative path of this entry.
    pub fn relative_path(&self) -> &Utf8Path {
        match self {
            Self::WalkDir { relative_path, .. } => relative_path,
            Self::Rgssad { file } => Utf8Path::new(file.name()),
        }
    }
}

impl Read for FileEntry<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::WalkDir { file, .. } => file.read(buffer),
            Self::Rgssad { file } => file.read(buffer),
        }
    }
}
