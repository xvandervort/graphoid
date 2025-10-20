//! Graphoid CLI and REPL

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
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
        Ok(source) => {
            let mut executor = Executor::new();

            match execute_source(&source, &mut executor) {
                Ok(_) => {
                    // File executed successfully
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() {
    println!("Graphoid v0.1.0");
    println!("Type /exit to quit, /help for help");

    let mut executor = Executor::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();

        // Handle REPL commands
        if input == "/exit" || input == "/quit" {
            break;
        }

        if input == "/help" {
            print_help();
            continue;
        }

        if input.is_empty() {
            continue;
        }

        // Execute the input
        match execute_repl_line(input, &mut executor) {
            Ok(Some(value)) => {
                println!("{}", value);
            }
            Ok(None) => {
                // Statement executed successfully, no value to print
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    println!("Goodbye!");
}

/// Execute source code from a file
fn execute_source(source: &str, executor: &mut Executor) -> Result<(), String> {
    // Tokenize
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    // Parse
    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    // Execute each statement
    for stmt in &program.statements {
        // Ignore return value (top-level returns shouldn't happen in file execution)
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(())
}

/// Execute a single line in REPL mode
/// Returns Ok(Some(value)) for expressions, Ok(None) for statements
fn execute_repl_line(
    source: &str,
    executor: &mut Executor,
) -> Result<Option<graphoid::values::Value>, String> {
    // Tokenize
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    // Parse
    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    // In REPL, if we have a single expression statement, return its value
    if program.statements.len() == 1 {
        if let graphoid::ast::Stmt::Expression { expr, .. } = &program.statements[0] {
            let value = executor
                .eval_expr(expr)
                .map_err(|e| format!("Runtime error: {}", e))?;
            return Ok(Some(value));
        }
    }

    // Otherwise, execute all statements
    for stmt in &program.statements {
        // Ignore return value (top-level returns in REPL are rare but allowed)
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(None)
}

fn print_help() {
    println!("Graphoid REPL Commands:");
    println!("  /exit, /quit - Exit the REPL");
    println!("  /help        - Show this help message");
    println!();
    println!("Examples:");
    println!("  > 2 + 3");
    println!("  > x = 10");
    println!("  > x * 2");
    println!("  > [1, 2, 3]");
    println!("  > \"hello\" + \" world\"");
}
