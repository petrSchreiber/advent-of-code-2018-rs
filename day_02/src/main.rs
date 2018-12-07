use std::io::BufRead;
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
            let ids = match get_ids(input_file) {
                Ok(v) => v,
                Err(error) => {
                                println!("Error found while retrieving ids from {}:\n{}", input_file, error);
                                return
                            }
            };

            let checksum = part01::get_vector_checksum(&ids);
            println!("The resulting checksum is {}", checksum);

            let common = part02::get_common_chars(&ids);
            println!("The common letter between the two IDs are {}", common);

        },

        _ => {
            println!("Please supply just 1 parameter: input file");
        }
    }
}

fn get_ids(input_file: &str) -> Result<std::vec::Vec<String>, std::io::Error> {
    let input = match std::fs::File::open(input_file) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let buffered = std::io::BufReader::new(input);

    let mut lines: std::vec::Vec<String> = std::vec::Vec::new();

    for line in buffered.lines() {
        lines.push(line.unwrap().to_string());
    }

    Ok(lines)
}
