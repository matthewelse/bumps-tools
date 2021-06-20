// Different years have different numbers of days, so we need to extract that
// from data files.

use std::collections::HashMap;
use std::io::Read;
use std::num::ParseIntError;

pub struct Year {
    pub days: u8,
    // crews_per_division: Vec<u8>,
}

impl Year {
    pub fn from_file(file: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let mut result = String::new();

        file.read_to_string(&mut result)?;

        let pairs: HashMap<String, String> = result
            .split('\n')
            .filter_map(|x| x.trim().split_once(' '))
            .map(|(x, y)| (String::from(x), String::from(y)))
            .collect();

        let days = pairs["DAYS:"].parse::<u8>()?;
        let crews_per_division: Result<Vec<u8>, ParseIntError> =
            pairs["DIVS:"].split(',').map(|x| x.parse::<u8>()).collect();

        let _crews_per_division = match crews_per_division {
            Ok(crews_per_division) => crews_per_division,
            Err(err) => return Err(Box::new(err)),
        };

        Ok(Year {
            days,
            // crews_per_division: crews_per_division,
        })
    }
}
