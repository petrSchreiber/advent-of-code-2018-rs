use std::{
    str::FromStr,
    io::BufRead
};

mod part01;
mod part02;

fn main() {
    let args: std::vec::Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            println!("Please supply input file as parameter");
        },

        2 => {
            let input_file = &args[1];
            let increments = match get_increments(input_file) {
                Ok(v) => v,
                Err(error) => {
                                println!("Error found while retrieving increments from {}:\n{}", input_file, error);
                                return
                            }
            };

            let frequency = part01::get_frequency(&increments);
            println!("The resulting frequency is {}", frequency);

            let repeated_frequency = part02::get_repeated_frequency(&increments);
            println!("The first frequency my device reaches twice is {}", repeated_frequency);            
        },

        _ => {
            println!("Please supply just 1 parameter: input file");
        }
    }
}

fn get_increments(input_file: &str) -> Result<std::vec::Vec<i32>, std::io::Error> {
    let input = match std::fs::File::open(input_file) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let buffered = std::io::BufReader::new(input);

    let mut increments: std::vec::Vec<i32> = std::vec::Vec::new();

    for line in buffered.lines() {
        let line_str = &line.unwrap();
        let increment = i32::from_str(line_str).unwrap();
        increments.push(increment);
    }

    Ok(increments)
}
