use anyhow::ensure;
use anyhow::Context;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

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
    input: PathBuf,

    #[argh(positional, description = "the folder to unpack to")]
    output: PathBuf,
}

pub fn exec(mut options: Options) -> anyhow::Result<()> {
    options.input = options
        .input
        .canonicalize()
        .context("failed to canonicalize input path")?;

    // TODO: Should we validate the output in some way?
    // Prevent writing to the input dir? Ensure it exists?
    std::fs::create_dir_all(&options.output)?;

    let mut file_entry_iter = FileEntryIter::new(&options.input)?;

    while let Some(entry) = file_entry_iter.next_file_entry()? {
        let relative_path = entry.relative_path();
        let output_path = options.output.join(relative_path);

        eprintln!("Extracting \"{relative_path}\"");

        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}

enum FileEntryIter {
    WalkDir {
        input_path: PathBuf,
        iter: walkdir::IntoIter,
    },
    Rgssad {
        reader: rgssad::Reader<File>,
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
            return Self::new_rgssad_path(path);
        }

        let rgssad_path = path.join("Game.rgssad");
        match File::open(&rgssad_path) {
            Ok(file) => {
                return Self::new_rgssad_file(file);
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
        let iter = WalkDir::new(path).into_iter();

        Ok(FileEntryIter::WalkDir {
            input_path: path.into(),
            iter,
        })
    }

    /// Create a new iter from the given rgssad path.
    pub fn new_rgssad_path<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let file = File::open(path)
            .with_context(|| format!("failed to open input file from \"{}\"", path.display()))?;
        Self::new_rgssad_file(file)
    }

    /// Create a new iter from the given rgssad file.
    pub fn new_rgssad_file(file: File) -> anyhow::Result<Self> {
        let mut reader = rgssad::Reader::new(file);
        reader.read_header()?;

        Ok(Self::Rgssad { reader })
    }

    /// Get the next file entry.
    pub fn next_file_entry(&mut self) -> anyhow::Result<Option<FileEntry>> {
        match self {
            Self::WalkDir { input_path, iter } => {
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
                let relative_path = entry_path.strip_prefix(&input_path)?;
                let relative_path = <&Utf8Path>::try_from(relative_path)?;

                Ok(Some(FileEntry::WalkDir {
                    relative_path: relative_path.into(),
                    file,
                }))
            }
            Self::Rgssad { reader } => {
                let entry = match reader.read_entry()? {
                    Some(entry) => entry,
                    None => return Ok(None),
                };

                Ok(Some(FileEntry::Rgssad { entry }))
            }
        }
    }
}

enum FileEntry<'a> {
    WalkDir {
        relative_path: Utf8PathBuf,
        file: File,
    },
    Rgssad {
        entry: rgssad::reader::Entry<'a, File>,
    },
}

impl FileEntry<'_> {
    fn relative_path(&self) -> &Utf8Path {
        match self {
            Self::WalkDir { relative_path, .. } => relative_path,
            Self::Rgssad { entry } => entry.file_name().into(),
        }
    }
}

impl Read for FileEntry<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::WalkDir { file, .. } => file.read(buffer),
            Self::Rgssad { entry } => entry.read(buffer),
        }
    }
}
