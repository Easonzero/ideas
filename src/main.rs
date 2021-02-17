mod error;
mod interaction;
mod item;
mod status;
mod store;

use clap::{clap_app, crate_authors, crate_description, crate_version};
use crossterm::style::Colorize;
use error::Result;
use interaction::{Interaction, Is, Op, IC};
use status::TALL;
use std::io::{stdin, stdout};
use store::{ItemPair, Store};

const DATA_DIR: &'static str = ".config/ideas";

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
        (@arg SHORT: --short "write idea quickly, skip detail and related url")
        (@subcommand view =>
            (about: "list ideas")
            (@arg TAG: -t --tag [TAG]... possible_values(TALL) "filter ideas by tag"))
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
        Some(("view", submatches)) => {
            let tags;
            if let Some(iter) = submatches.values_of("TAG") {
                tags = iter.collect::<Vec<_>>();
            } else {
                tags = TALL.to_vec();
            }
            let mut items: Vec<_> = store
                .iter()
                .map(|x| x.unwrap())
                .filter(|x| tags.contains(&x.item.ty.desc().as_str()))
                .collect();
            items.sort_unstable_by(|a, b| b.item.time.partial_cmp(&a.item.time).unwrap());
            let ItemPair { id, mut item } = interaction.view_items(items)?;
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
        _ => {
            let skip = matches.is_present("SHORT");
            let item = interaction.fill_item(skip)?;
            store.insert(&item)?;
            Ok(())
        }
    }
}
