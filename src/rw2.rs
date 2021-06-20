use nom::{do_parse, many0, named, number::complete::le_u32};
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub struct Details {
    years_active: (u32, u32),
    pub indices: (u32, u32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Row {
    DidNotCompete,
    Competed(Details),
}

named!(
    row<Vec<Option<Row>>>,
    many0!(do_parse!(
        _zero: le_u32
            >> _zero: le_u32
            >> start_year: le_u32
            >> end_year: le_u32
            >> start_idx: le_u32
            >> end_idx: le_u32
            >> (if start_year == 0 {
                None
            } else if start_year == 9999 {
                Some(Row::DidNotCompete)
            } else {
                Some(Row::Competed(Details {
                    years_active: (start_year, end_year),
                    indices: (start_idx, end_idx),
                }))
            })
    ))
);

impl Row {
    pub fn from_file(rw2: &Path) -> Result<Vec<Self>, Error> {
        let mut s = Vec::new();
        let mut rw2 = File::open(rw2)?;
        let _bytes_read = rw2.read_to_end(&mut s)?;

        match row(&s) {
            Ok((_, rows)) => Ok(rows.into_iter().flatten().collect()),
            Err(_) => Err(Error::new(ErrorKind::Other, "Unable to parse rw2 file.")),
        }
    }

    pub fn start_year(&self) -> Option<u32> {
        match self {
            Self::DidNotCompete => None,
            Self::Competed(details) => Some(details.years_active.0),
        }
    }

    pub fn end_year(&self) -> Option<u32> {
        match self {
            Self::DidNotCompete => None,
            Self::Competed(details) => Some(details.years_active.1),
        }
    }

    pub fn start_idx(&self) -> Option<u32> {
        match self {
            Self::DidNotCompete => None,
            Self::Competed(details) => Some(details.indices.0),
        }
    }

    pub fn end_idx(&self) -> Option<u32> {
        match self {
            Self::DidNotCompete => None,
            Self::Competed(details) => Some(details.indices.1),
        }
    }
}
