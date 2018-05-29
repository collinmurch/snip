extern crate isatty;

use std::io::stdin;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

use isatty::stdin_isatty;

struct Arguments {
    text: String,
    flag: String,
    delimiter: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() > 4 {
            return Err("too many arguments");
        }

        let mut delimiter = String::from(" ");
        let mut text = String::new();
        let mut flag = if args.len() > 1 {
            args[1].clone()
        } else {
            String::new()
        };
        
        if flag.contains("-h") || flag.contains("-help") {
            return Err("help");
        }
    
        // If user is using a file (stdin is empty)
        if stdin_isatty() {
            if args.len() < 2 || args.len() == 3 {
                return Err("no stdin supplied")
            }

            let f = if args.len() == 2 {
                args[1].clone()       
            } else {
                flag = args[1].clone();
                delimiter = args[2].clone();

                args[3].clone()
            };


            if let Ok(mut file) = File::open(f) {
                let mut text = String::new();

                match file.read_to_string(&mut text) {
                    Ok(_) => return Ok(Arguments{text, flag, delimiter}),
                    Err(_) => return Err("could not parse file"),
                };
            } else {
                return Err("could not find file");
            }

        // If user is using stdin 
        } else {
            let input = stdin();
            match input.read_line(&mut text) {
                Ok(_) => {
                    if args.len() == 2 {
                        return Err("stdin supplied, but too many arguments")
                    }
                },
                Err(_) => return Err("could not read stdin"),
            };

            if args.len() == 1 {
                delimiter = String::from(" ");

                return Ok(Arguments{text, flag, delimiter})
            } 
            else {
                flag = args[1].clone();

                delimiter = args[2].clone();

                return Ok(Arguments{text, flag, delimiter});
            }
        }
    }
}

fn snip(text: String, delimiter: &str) -> Vec<String> {
    let split: Vec<&str> = text.split(delimiter).collect();

    let mut new: Vec<String> = Vec::new();

    for item in split {
        new.push(item.to_string());
    }

    return new;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                println!("TO DO.");
                exit(0);
            } else {
                eprintln!("Problem parsing arguments: {}", err);
                exit(0);
            }
        }
    );

    let cut: Vec<String> = snip(arguments.text, &arguments.delimiter[..]);

    println!("RESULTS: ");
    for item in cut {
        println!("{}", item);
    }
}