use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

use crate::interaction::Searchable;

pub const UNDONE: char = '❎';
pub const DONE: char = '✅';
pub const TTODO: &'static str = "todo";
pub const IDEA: char = '🧠';
pub const TIDEA: &'static str = "idea";
pub const TIPS: char = '💡';
pub const TTIPS: &'static str = "tips";

pub const TALL: &[&str; 3] = &[TTODO, TIDEA, TTIPS];

#[derive(Clone, Serialize, Deserialize)]
pub struct Type {
    status: char,
    desc: String,
}

impl Type {
    pub fn new(status: char) -> Type {
        Type {
            status,
            desc: status2desc(status).to_owned(),
        }
    }
    pub fn status(&self) -> char {
        self.status
    }
    pub fn next_status(&self) -> Option<char> {
        match self.status {
            UNDONE => Some(DONE),
            _ => None,
        }
    }
    pub fn last_status(&self) -> Option<char> {
        match self.status {
            DONE => Some(UNDONE),
            _ => None,
        }
    }
    pub fn desc(&self) -> &String {
        &self.desc
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad(format!("{} {}", self.status, self.desc).as_str())
    }
}

impl Searchable for Type {
    fn is_match(&self, pat: &String) -> bool {
        self.desc.contains(pat)
    }
}

fn status2desc(s: char) -> &'static str {
    match s {
        UNDONE | DONE => TTODO,
        IDEA => TIDEA,
        TIPS => TTIPS,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_basis() {
        let t = Type::new(UNDONE);
        assert_eq!(t.next_status(), Some(DONE));
        assert_eq!(t.desc(), &TTODO.to_owned());
    }
}
