use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

/// An abstraction of a file sink over dir and rgssad output formats.
#[derive(Debug)]
pub enum FileSink {
    Dir { base_path: PathBuf },
    Rgssad { writer: rgssad::Writer<File> },
}

impl FileSink {
    /// Create a new file sink for a directory
    pub fn new_dir(path: &Path) -> anyhow::Result<Self> {
        std::fs::create_dir_all(path)?;

        // TODO: Maybe use a dir lock?

        Ok(Self::Dir {
            base_path: path.into(),
        })
    }

    /// Create a new file sink for an rgssad file
    pub fn new_rgssad(path: &Path) -> anyhow::Result<Self> {
        // TODO: Lock the file?

        let file = File::create_new(path)?;
        let mut writer = rgssad::Writer::new(file);
        writer.write_header()?;

        Ok(Self::Rgssad { writer })
    }

    /// Write a file.
    pub fn write_file<R>(
        &mut self,
        path_components: &[&str],
        size: u32,
        mut reader: R,
    ) -> anyhow::Result<()>
    where
        R: Read,
    {
        match self {
            Self::Dir { base_path } => {
                let mut path = base_path.clone();
                path.extend(path_components);

                // TODO: Temp paths.
                let mut file = File::create_new(path)?;
                std::io::copy(&mut reader, &mut file)?;
                file.flush()?;
                file.sync_all()?;
            }
            Self::Rgssad { writer } => {
                // Create a windows-style path.
                let path = path_components.join("\\");

                writer.write_entry(&path, size, reader)?;
            }
        }

        Ok(())
    }

    /// Finish and close this file sink.
    pub fn finish(&mut self) -> anyhow::Result<()> {
        match self {
            Self::Dir { .. } => {}
            Self::Rgssad { writer } => {
                let file = writer.get_mut();
                file.flush()?;
                file.sync_all()?;
            }
        }

        Ok(())
    }
}
