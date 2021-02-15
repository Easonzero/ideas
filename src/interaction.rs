mod icore;
mod iview;

use crate::error::*;
use crate::item::*;
use crate::status::*;
use crate::store::ItemPair;
use icore::Core;
use iview::View;
use std::{
    fmt::{self, Debug, Display, Formatter},
    io::{StdinLock, StdoutLock},
};

pub type IC<'a> = Core<StdinLock<'a>, StdoutLock<'a>>;

#[derive(Clone, Debug)]
pub enum Op {
    Cancel,
    Update,
    Retrieve,
    Delete,
}

#[derive(Clone, Debug)]
pub enum Is {
    Yes,
    No,
}

macro_rules! display_enum {
    ($id: ident) => {
        impl Display for $id {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Debug::fmt(self, f)
            }
        }
    };
}

display_enum!(Op);
display_enum!(Is);

pub struct Interaction<'a> {
    core: IC<'a>,
    view: View,
}

impl<'a> Interaction<'a> {
    pub fn new(core: IC<'a>) -> Self {
        Interaction {
            core,
            view: View::new(),
        }
    }

    pub fn confirm_again(&mut self) -> Result<Is> {
        self.core.question("? Do you confirm", "<required>")?;
        let is = self
            .core
            .read_input_from(vec![Is::No, Is::Yes], icore::Direction::Horizontal)?;
        Ok(is)
    }

    pub fn curd(&mut self) -> Result<Op> {
        self.core
            .question("? Please select your operator", "<required>")?;
        let op = self.core.read_input_from(
            vec![Op::Retrieve, Op::Update, Op::Delete, Op::Cancel],
            icore::Direction::Horizontal,
        )?;
        Ok(op)
    }

    pub fn view_items(&mut self, items: Vec<ItemPair>) -> Result<ItemPair> {
        if items.is_empty() {
            return Err(Error::StringError("Nothing here".to_string()));
        }
        self.core
            .question("? Please select the idea you want to operate", "<required>")?;
        let item = self
            .core
            .read_input_from(items, icore::Direction::Vertical)?;
        Ok(item)
    }

    pub fn view_item(&mut self, item: Item) -> Result<()> {
        self.view.run(item);
        Ok(())
    }

    pub fn update_item(&mut self, mut item: Item) -> Result<Item> {
        let mut status = vec![item.ty.status];
        next(item.ty.status).map(|s| status.push(s));
        last(item.ty.status).map(|s| status.push(s));
        if status.len() > 1 {
            self.core
                .question("? Please select status of the idea", "<required>")?;
            let status = self
                .core
                .read_input_from(status, icore::Direction::Horizontal)?;
            item.ty.status = status;
        }

        self.core.question("? Please enter your idea", "<option>")?;
        item.summary = self.core.read_input_with(Some(item.summary))?.unwrap();
        self.core
            .question("? Please enter description of the idea", "[option]")?;
        item.detail = self.core.read_input_with(item.detail)?;
        self.core
            .question("? Please enter related url of the idea", "[option]")?;
        item.url = self.core.read_input_with(item.url)?;
        Ok(item)
    }

    pub fn fill_item(&mut self) -> Result<Item> {
        self.core
            .question("? Please select type of the idea", "<required>")?;
        let ty = self.core.read_input_from(
            vec![
                Type {
                    status: UNDONE,
                    desc: "todo".to_owned(),
                },
                Type {
                    status: IDEA,
                    desc: "idea".to_owned(),
                },
                Type {
                    status: TIPS,
                    desc: "tips".to_owned(),
                },
            ],
            icore::Direction::Horizontal,
        )?;

        self.core
            .question("? Please enter your idea", "<required>")?;
        let summary = self
            .core
            .read_input()
            .map_err(|e| Error::from(e))
            .and_then(|idea| match idea {
                Some(idea) => Ok(idea),
                _ => Err(Error::StringError(
                    "the field `idea` is required!".to_string(),
                )),
            })?;
        self.core
            .question("? Please enter description of the idea", "[option]")?;
        let detail = self.core.read_input()?;
        self.core
            .question("? Please enter related url of the idea", "[option]")?;
        let url = self.core.read_input()?;
        Ok(Item {
            ty,
            summary,
            detail,
            url,
        })
    }
}
