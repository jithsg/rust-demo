
use immutable::get_fruits;

use clap::{Command, Arg};
use std::convert::TryInto;

fn main() {
    let matches = Command::new("Fruit Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates random fruits")
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_name("COUNT")
                .help("The quantity of fruits to be returned")
        )
        .get_matches();

    let count = matches.get_one::<String>("count")
                       .map(|s| s.parse::<usize>().expect("Count must be a valid integer"))
                       .unwrap_or(1);

    // Convert usize to u32 with error handling
    let count_u32: u32 = count.try_into().expect("Count value is too large for u32");

    let fruits = get_fruits(count_u32);
    for fruit in fruits {
        println!("{}", fruit);
    }
}
