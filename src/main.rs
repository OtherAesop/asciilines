// Copyright © 2019 Ian Gore
// [This program is licensed under the "GNU GPL 3 License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::env::args;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

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

pub fn read_file(filepath: &str) -> Result<String,Error>{
    // File opening code from https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
    let path = Path::new(filepath);

    let mut file = File::open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn print_tvg(canvas: &Vec<String>, commands: &Vec<String>){

    let mut board = vec![vec!["."; canvas[0].parse().unwrap()]; canvas[1].parse().unwrap()];
    let mut curr_command = vec![];

    for command in commands{
        curr_command = command.split(" ").collect();
        println!("{:?}", curr_command);
        match curr_command[3] {
            "h" => {
                //let index = curr_command[2].parse::<isize>().unwrap();
                //for mut elem in &board[curr_command[2].parse::<usize>().unwrap()]{
                //    elem = &curr_command[0];
                //}
            },
            "v" => {},
            _   => panic!("unrecognized direction element in .tvg file"),
        }
    }

    for row in board{
        for element in row{
            print!("{}", element);
        }
        println!();
    }
}

/// Do basic error checking and render file
fn main() -> Result<(),Error>{

    let args: Vec<String> = args().collect();

    if args.len() == 2 {
        match parse_tvg(&args[1]){
            Ok(_) => {
                // Render .tvg
                let tvg = read_file(&args[1])?;
                let mut temp_commands: Vec<_> = tvg.split("\u{000D}\u{000A}").collect();
                temp_commands.reverse(); // We want to pop off the canvas size
                let mut temp_canvas: Vec<_> = match temp_commands.pop() {
                    Some(s) => (s as &str).split(' ').collect(),
                    None    => panic!("Empty file passed."),
                };
                temp_canvas.reverse(); // Put it in order
                temp_commands.reverse(); // Put it back in order

                let canvas: Vec<String> = temp_canvas.iter().map(|s| s.to_string()).collect();
                let commands: Vec<String> = temp_commands.iter().map(|s| s.to_string()).collect();

                print_tvg(&canvas, &commands);
            },
            Err(msg) => println!("Error encountered: {:?}", msg),
        }
    } else {
        println!("Usage: cargo run 'filepath-to-tvg'");
    }
    Ok(())
}
