use crate::status::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub ty: Type,
    pub summary: String,
    pub detail: Option<String>,
    pub url: Option<String>,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, " {}  {}", self.ty.status, self.summary)
    }
}
