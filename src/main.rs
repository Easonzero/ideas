mod error;
mod interaction;
mod item;
mod parser;
mod status;
mod store;

use clap::{clap_app, crate_authors, crate_description, crate_version};
use crossterm::style::Colorize;
use error::Result;
use interaction::{Interaction, Is, Op, IC};
use std::io::{stdin, stdout};
use store::{ItemPair, Store};

const DATA_DIR: &'static str = ".ideas";

fn main() {
    match main_throw_err() {
        Ok(_) => {}
        Err(e) => {
            let head = "error:".red();
            eprintln!("{} {}", head, e);
        }
    }
}

fn main_throw_err() -> Result<()> {
    let matches = clap_app!(ideas =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@subcommand view =>
            (about: "list ideas"))
        (@subcommand sync =>
            (about: "sync local with remote storage")
            (@arg REMOTE: <REMOTE> ... possible_values(&["notion"]) "the remote storage"))
    )
    .get_matches();

    let dir = home::home_dir()
        .map(|mut path| {
            path.push(DATA_DIR);
            path
        })
        .unwrap();
    let store = Store::open(&dir)
        .expect(format!("Load db fault! Try again after exec `rm -rf {:#?}`", &dir).as_str());

    let input = stdin();
    let output = stdout();
    let mut interaction = Interaction::new(IC::new(input.lock(), output.lock()));

    match matches.subcommand() {
        Some(("view", _matches)) => {
            let ItemPair { id, mut item } =
                interaction.view_items(store.iter().map(|x| x.unwrap()).collect())?;
            let op = interaction.curd()?;
            match op {
                Op::Retrieve => interaction.view_item(item)?,
                Op::Delete => match interaction.confirm_again()? {
                    Is::Yes => store.remove(id)?,
                    _ => {}
                },
                Op::Update => {
                    item = interaction.update_item(item)?;
                    store.update(id, &item)?
                }
                _ => {}
            }
            Ok(())
        }
        Some(("sync", _matches)) => unimplemented!(),
        _ => {
            let item = interaction.fill_item()?;
            store.insert(&item)?;
            Ok(())
        }
    }
}
