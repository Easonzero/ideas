use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Serialize, Deserialize)]
pub struct Type {
    pub status: char,
    pub desc: String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad(format!("{} {}", self.status, self.desc).as_str())
    }
}

pub const UNDONE: char = '❎';
pub const DONE: char = '✅';
pub const IDEA: char = '🧠';
pub const TIPS: char = '💡';

pub fn next(s: char) -> Option<char> {
    match s {
        UNDONE => Some(DONE),
        _ => None,
    }
}

pub fn last(s: char) -> Option<char> {
    match s {
        DONE => Some(UNDONE),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basis() {
        assert_eq!(next(UNDONE), Some(DONE));
        assert_eq!(next(DONE), None);
        assert_eq!(next(IDEA), None);
        assert_eq!(next(TIPS), None);
    }
}
