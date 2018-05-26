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
        let mut flag = args[1].clone();
        
        if args.len() > 4 {
            return Err("too many arguments");
        }
        else if flag.contains("-h") || flag.contains("-help") {
            return Err("help")
        }

        let mut text = String::new();
        let mut delimiter = String::new();

        // If user is using a file (stdin is empty)
        if stdin_isatty() {
            if args.len() < 2 || args.len() == 3 {
                return Err("no stdin supplied")
            }
            else if args.len() == 2 {
                let f = args[1].clone();
                
                if let Ok(mut file) = File::open(f) {
                    let mut text = String::new();

                    match file.read_to_string(&mut text) {
                        Ok(_) => return Ok(Arguments{text, flag, delimiter}),
                        Err(_) => return Err("could not parse file"),
                    };
                } else {
                    return Err("could not find file");
                }
            } else {
                flag = args[3].clone();

                return Ok(Arguments{text, flag, delimiter})
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
            
            return Ok(Arguments{text, flag, delimiter})
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                println!("TO DO.");
                exit(0);
            } else {
                eprintln!("Problem parsing arguments {}", err);
                exit(0);
            }
        }
    );
}
