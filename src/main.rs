/*
Rust playground commander.
*/

use clap::Clap;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    path::PathBuf,
};

mod colleges;
mod crew;
mod decode;
mod raw;
mod rw2;
mod year;

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

#[derive(Clap, Debug, PartialEq)]
enum Category {
    Men,
    Women,
}

#[derive(Clap, Debug, PartialEq)]
enum Competition {
    Early,
    Mays(Category),
    Lents(Category),
}

impl clap::ArgEnum for Competition {
    const VARIANTS: &'static [&'static str] = &[
        "early",
        "men-mays",
        "women-mays",
        "men-lents",
        "women-lents",
    ];

    fn from_str(str: &str, _: bool) -> Result<Self, String> {
        match str.to_lowercase().as_ref() {
            "early" => Ok(Self::Early),
            "men-mays" => Ok(Self::Mays(Category::Men)),
            "men-lents" => Ok(Self::Lents(Category::Men)),
            "women-lents" => Ok(Self::Lents(Category::Women)),
            "women-mays" => Ok(Self::Mays(Category::Women)),
            _ => Err(format!("{} is not a valid competition.", str)),
        }
    }
}

impl Competition {
    fn raw_name(&self) -> &'static str {
        match self {
            Self::Early => "early",
            Self::Mays(Category::Men) => "mays",
            Self::Mays(Category::Women) => "wmays",
            Self::Lents(Category::Men) => "lents",
            Self::Lents(Category::Women) => "wlents",
        }
    }

    fn charts_name(&self) -> &'static str {
        match self {
            Self::Early => "Early",
            Self::Mays(Category::Men) => "Mays",
            Self::Mays(Category::Women) => "WMays",
            Self::Lents(Category::Men) => "Lents",
            Self::Lents(Category::Women) => "WLents",
        }
    }
}

impl Display for Competition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Early => "early bumps",
            Self::Mays(Category::Men) => "men's may bumps",
            Self::Mays(Category::Women) => "women's may bumps",
            Self::Lents(Category::Men) => "men's may bumps",
            Self::Lents(Category::Women) => "women's may bumps",
        };

        f.write_str(text)
    }
}

#[derive(Clap, Debug)]
struct Query {
    #[clap(long, arg_enum)]
    competition: Competition,
    #[clap(short, long)]
    year: u16,
    #[clap(long)]
    crew: String,
    #[clap(long)]
    data_dir: PathBuf,
}

#[derive(Clap)]
enum Subcommand {
    Decode(Decode),
    Query(Query),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    match opts.subcommand {
        Subcommand::Decode(decode) => {
            let data = decode::decode(&decode.file)?;
            println!("{}", data);

            Ok(())
        }
        Subcommand::Query(query) => {
            let raw = query
                .data_dir
                .join(format!("Data/{}.raw", query.competition.raw_name()));

            let rw2 = query
                .data_dir
                .join(format!("Data/{}.rw2", query.competition.raw_name()));

            let colleges = query.data_dir.join("Data/College.dat");

            let mut colleges = decode::Decoder::new(std::fs::File::open(&colleges)?)?;

            let clubs = colleges::Clubs::from_file(&mut colleges)?;
            let crews = clubs.crews();
            let rows = rw2::Row::from_file(&rw2)?;
            let mut raw = raw::Raw::load(&raw)?;

            if rows.len() != crews.len() {
                return Err (Box::new(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,"Invalid input data: number of crews does not match the number expected.")));
            }

            let min_year = rows.iter().filter_map(|x| x.start_year()).min();
            let max_year = rows.iter().filter_map(|x| x.end_year()).max();

            let years = {
                match (min_year, max_year) {
                    (Some(min_year), Some(max_year)) => {
                        let years: Result<HashMap<u32, year::Year>, Box<dyn std::error::Error>> =
                            (min_year..=max_year)
                                .map(|year| {
                                    let path = query.data_dir.join(format!(
                                        "Charts/{}/{}.dat",
                                        query.competition.charts_name(),
                                        year
                                    ));

                                    let mut reader =
                                        decode::Decoder::new(std::fs::File::open(&path)?)?;

                                    let info = year::Year::from_file(&mut reader)?;

                                    Ok((year, info))
                                })
                                .collect();
                        years?
                    }

                    _ => HashMap::new(),
                }
            };

            let crews: HashMap<String, crew::CrewRecord> = rows
                .into_iter()
                .zip(crews)
                .filter_map(|(row, crew)| {
                    let years: BTreeMap<u32, Vec<u8>> = (row.start_year()?..=row.end_year()?)
                        .scan(row.start_idx()?, |start_idx, year| {
                            let this_start_idx = *start_idx;
                            let next_start_idx = this_start_idx + (years[&year].days as u32) + 1;
                            *start_idx = next_start_idx;

                            assert!((next_start_idx - 1) <= row.end_idx().unwrap());

                            let positions = raw.range(this_start_idx, next_start_idx).unwrap();

                            if positions.iter().all(|x| (*x == 0)) {
                                Some(None)
                            } else {
                                Some(Some((year, positions)))
                            }
                        })
                        .flatten()
                        .collect();

                    Some((
                        crew.alias.clone(),
                        crew::CrewRecord::new(crew.name.clone(), crew.alias.clone(), years),
                    ))
                })
                .collect();

            let crew = &crews.get(&query.crew);

            match crew {
                None => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Crew {} not found", query.crew),
                ))),
                Some(crew) => {
                    println!("year: {}", query.year);
                    println!("crew name: {}", crew.name);
                    println!("crew alias: {}", crew.alias);

                    match crew.year(query.year as u32) {
                        Some(results) => println!("results: {:?}", results),
                        None => println!("results: (did not compete)"),
                    }

                    Ok(())
                }
            }
        }
    }
}
