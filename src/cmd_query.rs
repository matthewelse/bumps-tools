use std::{error::Error, collections::{HashMap, BTreeMap}, path::PathBuf};

use clap::Parser;

use crate::{db_entry::Competition, decode, colleges, rw2, raw, crew, year};

#[derive(Parser, Debug)]
pub(crate) struct Query {
    #[arg(long, value_enum)]
    competition: Competition,
    #[arg(long)]
    min_year: u16,
    #[arg(long)]
    max_year: u16,
    #[arg(long)]
    crew: String,
    #[arg(long)]
    data_dir: PathBuf,
}

pub(crate) fn run(query : &Query) -> Result<(), Box<dyn Error>> {
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
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Invalid input data: number of crews does not match the number expected.",
        )));
    }

    let min_year = rows.iter().filter_map(|x| x.start_year()).min();
    let max_year = rows.iter().filter_map(|x| x.end_year()).max();

    let years = {
        match (min_year, max_year) {
            (Some(min_year), Some(max_year)) => {
                let years: Result<HashMap<u32, year::Year>, Box<dyn std::error::Error>> = (min_year
                    ..=max_year)
                    .map(|year| {
                        let path = query.data_dir.join(format!(
                            "Charts/{}/{}.dat",
                            query.competition.charts_name(),
                            year
                        ));

                        let mut reader = decode::Decoder::new(std::fs::File::open(&path)?)?;

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
            for year in query.min_year..=query.max_year {
                match crew.year(year as u32) {
                    Some(results) => println!("{year}: {:?}", results),
                    None => println!("results: (did not compete)"),
                }
            }

            Ok(())
        }
    }
}
