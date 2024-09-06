use anyhow::bail;
use anyhow::Context;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

/// An abstraction of a file sink over dir and rgssad output formats.
#[derive(Debug)]
pub enum FileSink {
    Dir {
        base_path: PathBuf,
    },
    Rgssad {
        writer: rgssad::Writer<BufWriter<File>>,
    },
}

impl FileSink {
    /// Create a new file sink for a directory
    pub fn new_dir(path: &Path, overwrite: bool) -> anyhow::Result<Self> {
        if path.try_exists()? {
            if overwrite {
                std::fs::remove_dir_all(path)?;
            } else {
                bail!("output path exists");
            }
        }

        std::fs::create_dir_all(path)
            .with_context(|| format!("failed to create dir at \"{}\"", path.display()))?;

        // TODO: Maybe use a dir lock?

        Ok(Self::Dir {
            base_path: path.into(),
        })
    }

    /// Create a new file sink for an rgssad file
    pub fn new_rgssad(path: &Path, overwrite: bool) -> anyhow::Result<Self> {
        if path.try_exists()? {
            if overwrite {
                std::fs::remove_file(path)?;
            } else {
                bail!("output path exists");
            }
        }

        // TODO: Lock the file?

        let file = File::create_new(path)?;
        let file = BufWriter::new(file);
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

                if let Some(parent_path) = path.parent() {
                    std::fs::create_dir_all(parent_path)?;
                }

                // TODO: Temp paths?
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
                let buf_writer = writer.get_mut();
                buf_writer.flush()?;

                let file = buf_writer.get_mut();
                file.sync_all()?;
            }
        }

        Ok(())
    }
}
