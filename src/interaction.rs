mod icore;

use crate::error::*;
use crate::item::*;
use crate::status::*;
use icore::Core;
use std::io::{StdinLock, StdoutLock};

pub type IC<'a> = Core<StdinLock<'a>, StdoutLock<'a>>;

pub struct Interaction<'a> {
    core: IC<'a>,
}

impl<'a> Interaction<'a> {
    pub fn new(core: IC<'a>) -> Self {
        Interaction { core }
    }

    pub fn fill_item(&mut self) -> Result<Item> {
        let mut builder = ItemBuilder::default();
        self.core
            .question("? Please enter your idea", "<required>")?;
        if let Some(idea) = self.core.read_input()? {
            builder.summary(idea);
        }
        self.core
            .question("? Please enter description of the idea", "[option]")?;
        if let Some(desc) = self.core.read_input()? {
            builder.detail(desc);
        }
        self.core
            .question("? Please enter related url of the idea", "[option]")?;
        if let Some(url) = self.core.read_input()? {
            builder.url(url);
        }
        self.core
            .question("? Please select status of the idea", "<required>")?;
        let ty = self.core.read_input_from(
            vec![
                Type::new(UNDONE, "todo"),
                Type::new(IDEA, "idea"),
                Type::new(TIPS, "tips"),
            ],
            icore::Direction::Horizontal,
        )?;
        builder.ty(ty).build().map_err(|e| Error::StringError(e))
    }
}
