/*
Rust playground commander.
*/

use clap::Clap;
use std::path::PathBuf;

mod decode;

#[derive(Clap)]
#[clap(version = "1.0", author = "Matthew Else <matthewelse1997@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(Clap)]
struct Decode {
    #[clap(short, long)]
    file: PathBuf,
}

#[derive(Clap)]
enum Subcommand {
    Decode(Decode),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    match opts.subcommand {
        Subcommand::Decode(decode) => {
            let data = decode::decode(&decode.file)?;
            println!("{}", data);

            Ok(())
        }
    }
}
