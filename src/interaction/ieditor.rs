use crate::Result;
use std::io::Write;
use std::{fs::File, process::Command};
use tempfile::TempDir;

pub fn read_from_editor(editor: impl AsRef<str>, hint: Option<String>) -> Result<Option<String>> {
    let temp_dir = TempDir::new()?;
    let temp_file = temp_dir.path().join("ideas.tempfile");

    if let Some(hint) = hint {
        let mut writer = File::create(&temp_file)?;
        write!(writer, "{}", hint)?;
    }

    Command::new(editor.as_ref())
        .arg(temp_file.to_str().unwrap())
        .status()?;

    let content = std::fs::read_to_string(&temp_file)?;

    temp_dir.close()?;

    Ok(if content != "" { Some(content) } else { None })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basis() {
        assert_eq!(read_from_editor("touch".to_owned(), None).unwrap(), None);
    }
}
