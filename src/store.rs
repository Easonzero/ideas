use std::{fmt, fmt::Display, fmt::Formatter, path::Path};

use crate::error::Result;
use crate::item::Item;

pub struct Store {
    db: sled::Db,
}

pub struct Iter {
    iter: sled::Iter,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Store> {
        Ok(Store {
            db: sled::open(path)?,
        })
    }
    pub fn insert(&self, item: &Item) -> Result<()> {
        let id = self.db.generate_id()?.to_string();
        self.update(id, item)
    }
    pub fn update(&self, id: String, item: &Item) -> Result<()> {
        self.db
            .insert(id, serde_json::to_string(item)?.into_bytes())?;
        Ok(())
    }
    #[allow(dead_code)]
    pub fn get(&self, id: String) -> Result<Option<Item>> {
        let value = self
            .db
            .get(id)?
            .map(|ref i_vec| AsRef::<[u8]>::as_ref(i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?;

        Ok(value
            .map(|v| serde_json::from_str(v.as_str()))
            .transpose()?)
    }
    #[allow(dead_code)]
    pub fn remove(&self, id: String) -> Result<()> {
        self.db.remove(id)?;
        self.db.flush()?;
        Ok(())
    }
    pub fn iter(&self) -> Iter {
        Iter {
            iter: self.db.iter(),
        }
    }
}

#[derive(Clone)]
pub struct ItemPair {
    pub id: String,
    pub item: Item,
}

impl Iterator for Iter {
    type Item = Result<ItemPair>;
    fn next(&mut self) -> Option<Self::Item> {
        let to_string = |v: &sled::IVec| String::from_utf8(AsRef::<[u8]>::as_ref(v).to_vec());
        self.iter.next().map(|x| {
            let (k, v) = x?;
            let id = to_string(&k)?;
            let item = serde_json::from_str(to_string(&v)?.as_str())?;
            Ok(ItemPair { id, item })
        })
    }
}

impl Display for ItemPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.item, f)
    }
}
