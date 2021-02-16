use crate::Result;
use std::process::Command;
use tempfile::TempDir;

pub fn read_from_editor(editor: impl AsRef<str>) -> Result<Option<String>> {
    let temp_dir = TempDir::new()?;
    let temp_file = temp_dir.path().join("ideas.tempfile");

    Command::new(editor.as_ref())
        .arg(temp_file.to_str().unwrap())
        .status()?;

    let content = std::fs::read_to_string(temp_file)?;

    temp_dir.close()?;

    Ok(if content != "" { Some(content) } else { None })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basis() {
        assert_eq!(read_from_editor("touch".to_owned()).unwrap(), None);
    }
}
