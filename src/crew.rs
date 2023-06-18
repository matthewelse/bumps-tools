use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub struct CrewRecord {
    pub name: String,  // long name, e.g. "King's 3"
    pub alias: String, // short name, e.g. "kings3"
    pub years: BTreeMap<u32, Vec<u8>>,
}

impl CrewRecord {
    pub fn new(name: String, alias: String, years: BTreeMap<u32, Vec<u8>>) -> Self {
        CrewRecord { name, alias, years }
    }

    pub fn year(&self, year: u32) -> Option<&Vec<u8>> {
        self.years.get(&year)
    }
}
