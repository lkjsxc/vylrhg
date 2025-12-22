use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("file is not valid UTF-8")]
    NotUtf8,

    #[error("path is a directory")]
    IsDirectory,
}

pub fn read_text(path: &Path) -> Result<String, FsError> {
    let meta = std::fs::metadata(path).map_err(FsError::Io)?;
    if meta.is_dir() {
        return Err(FsError::IsDirectory);
    }

    let bytes = std::fs::read(path).map_err(FsError::Io)?;
    let mut text = String::from_utf8(bytes).map_err(|_| FsError::NotUtf8)?;
    // Normalize newlines to \n.
    if text.contains("\r") {
        text = text.replace("\r\n", "\n");
        text = text.replace('\r', "\n");
    }
    Ok(text)
}

pub fn write_text(path: &Path, contents: &str) -> Result<(), FsError> {
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.is_dir() {
            return Err(FsError::IsDirectory);
        }
    }
    // MVP: overwrite directly.
    std::fs::write(path, contents.as_bytes()).map_err(FsError::Io)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_newlines() {
        let s = "a\r\nb\rc\n".to_string();
        let normalized = s.replace("\r\n", "\n").replace('\r', "\n");
        assert_eq!(normalized, "a\nb\nc\n");
    }
}
