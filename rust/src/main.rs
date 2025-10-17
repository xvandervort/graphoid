//! Graphoid CLI and REPL

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // File execution mode
        run_file(&args[1]);
    } else {
        // REPL mode
        run_repl();
    }
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(_source) => {
            println!("Executing: {}", path);
            // TODO: Execute the file
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    println!("Graphoid v0.1.0");
    println!("Type /exit to quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "/exit" {
            break;
        }

        // TODO: Execute input
        println!("TODO: Execute '{}'", input);
    }
}
