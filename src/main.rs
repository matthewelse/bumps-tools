/*
Rust playground commander.
*/

use clap::Parser;
use cmd_query::Query;
use std::path::PathBuf;

mod cmd_db;
mod cmd_query;
mod colleges;
mod crew;
mod db_entry;
mod decode;
mod raw;
mod rw2;
mod schema;
mod year;

#[derive(Parser)]
#[clap(version = "1.0", author = "Matthew Else <matthewelse1997@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser)]
struct Decode {
    #[clap(short, long)]
    file: PathBuf,
}

#[derive(Parser)]
enum Subcommand {
    Decode(Decode),
    Query(Query),
    #[clap(subcommand)]
    Db(cmd_db::Subcommand),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    match opts.subcommand {
        Subcommand::Decode(decode) => {
            let data = decode::decode(&decode.file)?;
            println!("{}", data);

            Ok(())
        }
        Subcommand::Query(query) => cmd_query::run(&query),
        Subcommand::Db(args) => cmd_db::run(&args),
    }
}
