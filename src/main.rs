// Copyright © 2019 Ian Gore
// [This program is licensed under the "GNU GPL 3 License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::env::args;

///! Render a .tvg file on the command line when passed as an argument
///! from standard input

/// Check passed file name to ensure it has a 'tvg' extension
pub fn parse_tvg(tvg: &str) -> Result<&str, &str>{
    let path = tvg.split('.').last();
    match path {
        Some("tvg") => Ok("tvg"),
        None        => Err("No file extension on passed file"), //Should not occur but included for completeness
        Some(_)     => Err("Unsupported file type"),
    }
}

/// Do basic error checking and render file
fn main() {

    let args: Vec<String> = args().collect();

    if args.len() == 2 {
        match parse_tvg(&args[1]){
            Ok(_) => {
                //Render .tvg
            },
            Err(msg) => println!("Error encountered: {:?}", msg),
        }
    } else {
        println!("Usage: cargo run 'filepath-to-tvg'");
    }
}
