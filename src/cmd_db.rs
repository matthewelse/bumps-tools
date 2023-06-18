use std::path::PathBuf;

use clap::Parser;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::db_entry::Competition::MenMays;
use crate::db_entry::Entry;
use crate::db_entry::NewEntry;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[arg(long)]
    data_dir: PathBuf,
    #[arg(long)]
    sqlite_path: Option<String>,
}

pub(crate) fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let sqlite_path = args.sqlite_path.clone().unwrap_or("bumps.db".into());

    let mut conn = SqliteConnection::establish(&sqlite_path)?;

    use crate::schema::entries::dsl::*;

    let results = entries.select(Entry::as_select()).load(&mut conn)?;

    println!("{results:?}");

    let new_entry = NewEntry {
        year: 2023,
        day: 4,
        club: "kings",
        crew: "kings",
        competition: MenMays,
        position: 4,
    };

    insert_into(crate::schema::entries::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    let results = entries.select(Entry::as_select()).load(&mut conn)?;

    println!("{results:?}");

    Ok(())
}
