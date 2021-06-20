// Algorithm for decoding DAT files included in the bumps installation.

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct Decoder<T: Read> {
    reader: T,
    key: u8,
}

impl<T: Read> Decoder<T> {
    pub fn new(mut reader: T) -> Result<Self, std::io::Error> {
        let mut buf = vec![0];

        let bytes_read = reader.read(&mut buf)?;

        if bytes_read == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Empty data file.",
            ));
        }

        let key = buf[0];

        Ok(Decoder { reader, key })
    }
}

impl<T: Read> Read for Decoder<T> {
    /// This implements the algorithm required to decode data files from the
    /// bumps CD-ROM. It is equivalent to splitting the file into lines, taking
    /// the first character of every line as a key, and modifying each
    /// subsequent character as follows:
    ///
    /// `c_i' = (c_i ^ (c_0 + i - 1)) & 0x7f`, where i is the offset from the
    /// start of the line.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let bytes_read = self.reader.read(buf)?;

        let mut skip = 0usize;
        let mut skip_next = false;

        for i in 0..bytes_read {
            if skip_next {
                skip_next = false;
                continue;
            }

            buf[i - skip] = buf[i];

            if buf[i] == b'\r' {
                continue;
            }

            if buf[i] == b'\n' {
                if (i + 1) < buf.len() {
                    self.key = buf[i + 1];

                    // we want to remove the key from the output buffer
                    skip_next = true;
                    skip += 1;
                }

                continue;
            }

            if buf[i] < 0x1f {
                // For whatever reason, they don't encrypt whitespace.
                continue;
            }

            buf[i - skip] = (buf[i - skip] ^ self.key) & 0x7f;

            self.key = self.key.wrapping_add(1);
        }

        Ok(bytes_read - skip)
    }
}

/// Decrypt the file at `path`, returning a string, or an error.
pub fn decode(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut decoded = BufReader::new(Decoder::new(File::open(path)?)?);
    let mut buf = String::new();

    decoded.read_to_string(&mut buf)?;

    Ok(buf)
}
