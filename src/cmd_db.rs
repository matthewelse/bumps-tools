use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::colleges;
use crate::colleges::Clubs;
use crate::crew::CrewRecord;
use crate::db_entry::Competition;
use crate::db_entry::NewEntry;
use crate::decode;
use crate::raw;
use crate::rw2;
use crate::year;

#[derive(Parser, Debug)]
pub(crate) enum Subcommand {
    FromBumpsCdRom(BuildArgs),
}

#[derive(Parser, Debug)]
pub(crate) struct BuildArgs {
    #[arg(long)]
    data_dir: PathBuf,
    #[arg(long)]
    sqlite_path: Option<String>,
}

fn load_clubs(data_dir: &Path) -> Result<Clubs, Box<dyn Error>> {
    let colleges = data_dir.join("Data/College.dat");

    let mut colleges = decode::Decoder::new(std::fs::File::open(&colleges)?)?;

    colleges::Clubs::from_file(&mut colleges)
}

fn load_years(
    data_dir: &Path,
    clubs: &Clubs,
    competition: Competition,
) -> Result<HashMap<String, CrewRecord>, Box<dyn Error>> {
    let raw = data_dir.join(format!("Data/{}.raw", competition.raw_name()));

    let rw2 = data_dir.join(format!("Data/{}.rw2", competition.raw_name()));

    let crews = clubs.crews();
    let rows = rw2::Row::from_file(&rw2)?;
    let mut raw = raw::Raw::load(&raw)?;

    if rows.len() != crews.len() {
        return Err(
            "Invalid input data: number of crews does not match the number expected.".into(),
        );
    }

    let min_year = rows
        .iter()
        .filter_map(|x| x.start_year())
        .min()
        .ok_or("Unable to find the minimum year.")?;
    let max_year = rows
        .iter()
        .filter_map(|x| x.end_year())
        .max()
        .ok_or("Unable to find the maximum year.")?;

    let years = {
        let years: Result<HashMap<u32, year::Year>, Box<dyn std::error::Error>> = (min_year
            ..=max_year)
            .map(|year| {
                let path =
                    data_dir.join(format!("Charts/{}/{}.dat", competition.charts_name(), year));

                let mut reader = decode::Decoder::new(std::fs::File::open(&path)?)?;

                let info = year::Year::from_file(&mut reader)?;

                Ok((year, info))
            })
            .collect();
        years?
    };

    Ok(rows
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
                CrewRecord::new(crew.name.clone(), crew.alias.clone(), years),
            ))
        })
        .collect())
}

fn from_bumps_cdrom(args: &BuildArgs) -> Result<(), Box<dyn std::error::Error>> {
    let sqlite_path = args.sqlite_path.clone().unwrap_or("bumps.db".into());

    let mut conn = SqliteConnection::establish(&sqlite_path)?;

    let clubs = load_clubs(&args.data_dir)?;

    let mut all_entries = vec![];

    for comp in [
        Competition::Early,
        Competition::MenLents,
        Competition::MenMays,
        Competition::WomenLents,
        Competition::WomenMays,
    ] {
        let crews = load_years(&args.data_dir, &clubs, comp).unwrap();

        for this_club in clubs.clubs() {
            for this_crew in &this_club.crews {
                if let Some(crew_record) = crews.get(&this_crew.alias) {
                    for (y, days) in &crew_record.years {
                        for (day_number, pos) in days.iter().enumerate() {
                            all_entries.push(NewEntry {
                                year: *y as i32,
                                day: day_number.try_into().unwrap(),
                                club: &this_club.name,
                                crew: &this_crew.alias,
                                competition: comp,
                                position: *pos as i32,
                            })
                        }
                    }
                }
            }
        }
    }

    println!("inserting {} entries", all_entries.len());

    insert_into(crate::schema::entries::table)
        .values(&all_entries)
        .execute(&mut conn)?;

    Ok(())
}

pub(crate) fn run(command: &Subcommand) -> Result<(), Box<dyn Error>> {
    match command {
        Subcommand::FromBumpsCdRom(args) => from_bumps_cdrom(&args),
    }
}
