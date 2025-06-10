mod util;

pub use self::util::guess_game_kind_from_exe;
use crate::GameKind;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

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
            let game_path = path.join("Game.exe");
            let game_exe = std::fs::read(&game_path)
                .with_context(|| format!("failed to read \"{}\"", game_path.display()))?;

            if let Some(game_kind) =
                guess_game_kind_from_exe(&game_exe).context("failed to guess game type")?
            {
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
                let entry = loop {
                    let entry = match iter.next() {
                        Some(Ok(entry)) => entry,
                        Some(Err(error)) => return Err(error).context("failed to read dir entry"),
                        None => return Ok(None),
                    };

                    // Rgssad archives only contain the "Data" and "Graphics" folders at the top level.
                    // Only include these folders for parity with rgssad archives.
                    if entry.depth() == 1
                        && ![OsStr::new("Data"), OsStr::new("Graphics")]
                            .contains(&entry.file_name())
                    {
                        if entry.file_type().is_dir() {
                            iter.skip_current_dir();
                        }
                        continue;
                    }

                    // Filter out dir entries, to keep similar behavior with rgssad.
                    if entry.file_type().is_dir() {
                        continue;
                    }

                    break entry;
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
