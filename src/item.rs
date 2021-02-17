use crate::{interaction::Searchable, status::*};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub ty: Type,
    pub summary: String,
    pub detail: Option<String>,
    pub url: Option<String>,
    pub time: std::time::SystemTime,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, " {}  {}", self.ty.status(), self.summary)
    }
}

impl Searchable for Item {
    fn is_match(&self, pat: &String) -> bool {
        self.ty.is_match(pat)
            || self.summary.contains(pat)
            || self
                .detail
                .as_ref()
                .map(|x| x.contains(pat))
                .unwrap_or(false)
    }
}
