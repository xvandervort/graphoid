//! Graphoid CLI - The `gr` command
//!
//! Usage:
//!   gr file.gr          Run a Graphoid file
//!   gr spec [path]      Run spec files (test runner)
//!   gr repl             Start interactive REPL
//!   gr version          Show version
//!   gr help             Show help

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::Value;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        run_repl();
        return;
    }

    match args[1].as_str() {
        "spec" => run_spec_command(&args[2..]),
        "repl" => run_repl(),
        "version" | "--version" | "-v" => println!("Graphoid v{}", VERSION),
        "help" | "--help" | "-h" => print_usage(),
        path if path.ends_with(".gr") => run_file(path),
        path if Path::new(path).exists() => run_file(path),
        unknown => {
            eprintln!("Unknown command or file: {}", unknown);
            eprintln!("Run 'gr help' for usage information.");
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Graphoid v{}", VERSION);
    println!();
    println!("Usage:");
    println!("  gr <file.gr>        Run a Graphoid program");
    println!("  gr spec [path]      Run spec files (discovers *_spec.gr)");
    println!("  gr repl             Start interactive REPL");
    println!("  gr version          Show version information");
    println!("  gr help             Show this help message");
    println!();
    println!("Examples:");
    println!("  gr myprogram.gr           Run a single file");
    println!("  gr spec                   Run all specs in current directory");
    println!("  gr spec tests/            Run all specs in tests/");
    println!("  gr spec tests/math_spec.gr  Run a specific spec file");
}

// =============================================================================
// Spec Runner - delegates to pure Graphoid implementation
// =============================================================================

fn run_spec_command(args: &[String]) {
    let path = if args.is_empty() { "." } else { &args[0] };

    let mut executor = Executor::new();

    // Set the path for the Graphoid spec runner
    let setup = format!("__SPEC_PATH__ = \"{}\"", path.replace('\\', "\\\\").replace('"', "\\\""));
    if let Err(e) = execute_source(&setup, &mut executor) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Run the pure Graphoid spec runner
    if let Err(e) = execute_source("import \"spec_runner\"", &mut executor) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Check the result
    let result_code = "__SPEC_RESULT__";
    let mut lexer = Lexer::new(result_code);
    if let Ok(tokens) = lexer.tokenize() {
        let mut parser = Parser::new(tokens);
        if let Ok(program) = parser.parse() {
            if let Some(stmt) = program.statements.first() {
                if let graphoid::ast::Stmt::Expression { expr, .. } = stmt {
                    if let Ok(value) = executor.eval_expr(expr) {
                        if let graphoid::values::ValueKind::Boolean(true) = value.kind {
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
    }
}

// =============================================================================
// File Runner
// =============================================================================

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(source) => {
            let mut executor = Executor::new();

            // Set current file for module resolution
            let abs_path = PathBuf::from(path).canonicalize()
                .unwrap_or_else(|_| PathBuf::from(path));
            executor.set_current_file(Some(abs_path));

            match execute_source(&source, &mut executor) {
                Ok(_) => {}
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

// =============================================================================
// REPL
// =============================================================================

fn run_repl() {
    println!("Graphoid v{}", VERSION);
    println!("Type /exit to quit, /help for help");

    let mut executor = Executor::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error reading input");
                continue;
            }
        }

        let input = input.trim();

        if input == "/exit" || input == "/quit" {
            break;
        }

        if input == "/help" {
            print_repl_help();
            continue;
        }

        if input.is_empty() {
            continue;
        }

        match execute_repl_line(input, &mut executor) {
            Ok(Some(value)) => println!("{}", value),
            Ok(None) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    println!("Goodbye!");
}

fn print_repl_help() {
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

// =============================================================================
// Execution Helpers
// =============================================================================

fn execute_source(source: &str, executor: &mut Executor) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    for stmt in &program.statements {
        executor.eval_stmt(stmt).map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(())
}

fn execute_repl_line(source: &str, executor: &mut Executor) -> Result<Option<Value>, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    if program.statements.len() == 1 {
        if let graphoid::ast::Stmt::Expression { expr, .. } = &program.statements[0] {
            let value = executor.eval_expr(expr).map_err(|e| format!("Runtime error: {}", e))?;
            return Ok(Some(value));
        }
    }

    for stmt in &program.statements {
        executor.eval_stmt(stmt).map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(None)
}
