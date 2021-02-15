mod error;
mod interaction;
mod item;
mod status;
mod store;

use clap::{clap_app, crate_authors, crate_description, crate_version};
use crossterm::style::Colorize;
use error::Result;
use interaction::{Interaction, IC};
use std::io::{stdin, stdout};
use store::Store;

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

    match matches.subcommand() {
        Some(("view", _matches)) => {
            let store = Store::open(&dir).expect(
                format!("Load db fault! Try again after exec `rm -rf {:#?}`", &dir).as_str(),
            );
            for x in store.iter() {
                let (_, v) = x?;
                println!("{}", v);
            }
            Ok(())
        }
        Some(("sync", _matches)) => unimplemented!(),
        _ => {
            let input = stdin();
            let output = stdout();
            let mut interaction = Interaction::new(IC::new(input.lock(), output.lock()));
            let item = interaction.fill_item()?;
            let store = Store::open(dir)?;
            store.insert(&item)?;
            Ok(())
        }
    }
}
