use std::io::Read;

#[derive(Debug)]
pub struct Crew {
    pub name: String,
    pub alias: String,
}

#[derive(Debug)]
pub struct Club {
    name: String,
    colour: (u8, u8, u8),
    crews: Vec<Crew>,
}

#[derive(Debug)]
pub struct Clubs(Vec<Club>);

impl Clubs {
    pub fn crews(&self) -> Vec<&Crew> {
        self.0.iter().flat_map(|club| club.crews.iter()).collect()
    }

    // Loads data from colleges.dat.
    pub fn from_file(file: &mut dyn Read) -> Result<Self, Box<dyn std::error::Error>> {
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        // File format:
        //
        // [name]
        // R G B
        // crew name     short_name
        // ...

        let mut clubs = vec![];

        let mut current_name = None;
        let mut current_colour = None;
        let mut current_crews = vec![];

        for line in contents.split('\n') {
            let line = line.trim();

            if line.starts_with('!') {
                continue;
            }

            if line.is_empty() {
                if let Some(current_name) = current_name {
                    if let Some(current_colour) = current_colour {
                        clubs.push(Club {
                            name: String::from(current_name),
                            colour: current_colour,
                            crews: current_crews,
                        });
                    }
                }

                current_name = None;
                current_colour = None;
                current_crews = vec![];
            } else if line.starts_with('[') && line.ends_with(']') {
                let inner = &line[1..(line.len() - 1)];
                current_name = Some(inner);
            } else if current_colour.is_none() {
                let rgb: Vec<u8> = line
                    .split('\t')
                    .map(|x| x.parse::<u8>())
                    .collect::<Result<Vec<u8>, _>>()?;

                let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

                current_colour = Some((r, g, b));
            } else if let Some((name, alias)) = line.rsplit_once('\t') {
                let name = name.trim();
                let alias = alias.trim();

                current_crews.push(Crew {
                    name: String::from(name),
                    alias: String::from(alias),
                });
            }
        }

        Ok(Clubs(clubs))
    }
}
