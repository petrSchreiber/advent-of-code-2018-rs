use core::str::FromStr;
use std::num::ParseIntError;
use std::io::BufRead;

pub struct Claim {
    pub id: usize,
    pub offset_x: usize,
    pub offset_y: usize,
    pub width: usize,
    pub height: usize
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // Input is "#<id> @ <x>,<y>: <width>x<height>"
        let mut parts = text.split(" ").collect::<Vec<&str>>();

        // Chopping the "#" from the beginning of Id
        parts[0] = &parts[0][1..];

        // Chopping the ":" out of coords part
        let part2_len = parts[2].len();
        parts[2] = &parts[2][0..part2_len-1];

        let id = parts[0].parse::<usize>()?;
        let coords: Vec<&str> = parts[2].split(",").collect();
        let area: Vec<&str> = parts[3].split("x").collect();
        
        let offset_x = coords[0].parse::<usize>()?;
        let offset_y = coords[1].parse::<usize>()?;

        let width = area[0].parse::<usize>()?;
        let height = area[1].parse::<usize>()?;

        Ok(Claim { id: id, offset_x: offset_x, offset_y: offset_y, width: width, height: height })
    }
}

pub fn get_claims(input_file: &str) -> Result<std::vec::Vec<Claim>, std::io::Error> {
    let input = match std::fs::File::open(input_file) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let buffered = std::io::BufReader::new(input);

    let mut lines: std::vec::Vec<Claim> = std::vec::Vec::new();

    for line in buffered.lines() {
        lines.push(Claim::from_str(&line.unwrap()).unwrap());
    }

    Ok(lines)
}
