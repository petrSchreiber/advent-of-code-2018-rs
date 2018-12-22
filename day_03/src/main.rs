mod claims;
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
            let claims = match claims::get_claims(input_file) {
                Ok(v) => v,
                Err(error) => {
                                println!("Error found while retrieving claims from {}:\n{}", input_file, error);
                                return
                            }
            };

            let overlapping_fabric = part01::get_overlapping_fabric(&claims);
            println!("The amount of overlapping fabric is {}", overlapping_fabric);

            let non_overlapping_id = part02::get_nonoverlapping_claim(&claims);
            println!("The non-overlapping claim is has id #{}", non_overlapping_id);

        },

        _ => {
            println!("Please supply just 1 parameter: input file");
        }
    }
}
