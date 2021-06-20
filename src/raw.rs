use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom};
use std::path::Path;

pub struct Raw {
    read: File,
}

impl Raw {
    pub fn load(raw: &Path) -> Result<Self, Error> {
        let reader = File::open(raw)?;

        Ok(Raw { read: reader })
    }

    pub fn range(&mut self, start_incl: u32, end_excl: u32) -> Result<Vec<u8>, Error> {
        let mut result = vec![0; (end_excl - start_incl) as usize];
        self.read.seek(SeekFrom::Start(start_incl as u64))?;

        self.read.read_exact(&mut result)?;

        Ok(result)
    }
}
