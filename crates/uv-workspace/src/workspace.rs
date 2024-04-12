use std::path::{Path, PathBuf};

use crate::{Options, PyProjectToml};

/// Represents a project workspace that contains a set of options and a root path.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Workspace {
    options: Options,
    root: PathBuf,
}

impl Workspace {
    /// Find the [`Workspace`] for the given path.
    ///
    /// The search starts at the given path and goes up the directory tree until a workspace is
    /// found.
    pub fn find(path: impl AsRef<Path>) -> Result<Option<Self>, WorkspaceError> {
        for ancestor in path.as_ref().ancestors() {
            if let Some(options) = read_options(ancestor)? {
                return Ok(Some(Self {
                    options,
                    root: ancestor.to_path_buf(),
                }));
            }
        }
        Ok(None)
    }
}

/// Read a `uv.toml` or `pyproject.toml` file in the given directory.
fn read_options(path: &Path) -> Result<Option<Options>, WorkspaceError> {
    // Read a `uv.toml` file in the current directory.
    match fs_err::read_to_string(path.join("uv.toml")) {
        Ok(content) => {
            let options: Options = toml::from_str(&content)?;
            return Ok(Some(options));
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }

    // Read a `pyproject.toml` file in the current directory.
    match fs_err::read_to_string(path.join("pyproject.toml")) {
        Ok(content) => {
            let pyproject: PyProjectToml = toml::from_str(&content)?;
            return Ok(pyproject.tool.and_then(|tool| tool.uv));
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }

    Ok(None)
}

#[derive(thiserror::Error, Debug)]
pub enum WorkspaceError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}
