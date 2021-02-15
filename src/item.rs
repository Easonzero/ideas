use crate::status::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Builder, Serialize, Deserialize)]
pub struct Item {
    ty: Type,
    summary: String,
    #[builder(default, setter(strip_option))]
    detail: Option<String>,
    #[builder(default, setter(strip_option))]
    url: Option<String>,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, " {}  {}", self.ty.status(), self.summary)
    }
}
