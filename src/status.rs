use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Serialize, Deserialize)]
pub struct Type(char, String);

impl Type {
    pub fn new(s: char, desc: impl ToString) -> Self {
        Type(s, desc.to_string())
    }
    pub fn status(&self) -> &char {
        &self.0
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad(format!("{} {}", self.0, self.1).as_str())
    }
}

#[allow(dead_code)]
pub const UNDONE: char = '❎';
pub const DONE: char = '✅';
pub const IDEA: char = '🧠';
pub const TIPS: char = '💡';

#[allow(dead_code)]
fn next(s: char) -> char {
    match s {
        UNDONE => DONE,
        DONE => UNDONE,
        _ => s,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basis() {
        assert_eq!(next(UNDONE), DONE);
        assert_eq!(next(DONE), UNDONE);
        assert_eq!(next(IDEA), IDEA);
        assert_eq!(next(TIPS), TIPS);
    }
}
