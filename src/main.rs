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
// Spec Runner
// =============================================================================

fn run_spec_command(args: &[String]) {
    let path = if args.is_empty() {
        PathBuf::from(".")
    } else {
        PathBuf::from(&args[0])
    };

    let spec_files = discover_spec_files(&path);

    if spec_files.is_empty() {
        eprintln!("No spec files found in {:?}", path);
        eprintln!("Spec files must end with _spec.gr");
        std::process::exit(1);
    }

    println!("Running {} spec file(s)...\n", spec_files.len());

    // Single executor for all spec files - Graphoid handles everything
    let mut executor = Executor::new();

    // Initialize the gspec framework once
    if let Err(e) = inject_spec_dsl(&mut executor) {
        eprintln!("Error initializing spec runner: {}", e);
        std::process::exit(1);
    }

    // Run each spec file
    for spec_file in &spec_files {
        let abs_path = spec_file.canonicalize().unwrap_or_else(|_| spec_file.to_path_buf());
        executor.set_current_file(Some(abs_path));

        match fs::read_to_string(spec_file) {
            Ok(source) => {
                if let Err(e) = execute_source(&source, &mut executor) {
                    eprintln!("\nError in {}: {}", spec_file.display(), e);
                }
            }
            Err(e) => {
                eprintln!("\nError reading {}: {}", spec_file.display(), e);
            }
        }
    }

    // Let Graphoid print the final summary and tell us if tests failed
    let has_failures = finalize_spec_run(&mut executor);

    if has_failures {
        std::process::exit(1);
    }
}

fn finalize_spec_run(executor: &mut Executor) -> bool {
    // Call __spec_runner__.final_summary() which prints summary and returns true if failures
    let code = "__spec_runner__.final_summary()";

    let mut lexer = Lexer::new(code);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return true,
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(_) => return true,
    };

    if let Some(stmt) = program.statements.first() {
        if let graphoid::ast::Stmt::Expression { expr, .. } = stmt {
            if let Ok(value) = executor.eval_expr(expr) {
                // Returns true if there were failures
                if let graphoid::values::ValueKind::Boolean(b) = value.kind {
                    return b;
                }
            }
        }
    }

    true // Assume failure if we can't determine
}

fn discover_spec_files(path: &Path) -> Vec<PathBuf> {
    let mut spec_files = Vec::new();

    if path.is_file() {
        // Single file specified
        if path.to_string_lossy().ends_with("_spec.gr") {
            spec_files.push(path.to_path_buf());
        }
    } else if path.is_dir() {
        // Directory - find all *_spec.gr files recursively
        discover_spec_files_recursive(path, &mut spec_files);
    }

    spec_files.sort();
    spec_files
}

fn discover_spec_files_recursive(dir: &Path, spec_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Skip hidden directories and common non-source dirs
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if !name.starts_with('.') && name != "target" && name != "node_modules" {
                    discover_spec_files_recursive(&path, spec_files);
                }
            } else if path.is_file() {
                if path.to_string_lossy().ends_with("_spec.gr") {
                    spec_files.push(path);
                }
            }
        }
    }
}

fn inject_spec_dsl(executor: &mut Executor) -> Result<(), String> {
    // Create the test runner and DSL by executing gspec setup code
    let setup_code = r#"
import "gspec"

# Create the global test runner
__spec_runner__ = gspec.TestRunner.clone()

# DSL functions that delegate to the runner
fn describe(name, block) {
    __spec_runner__.describe(name, block)
}

fn context(name, block) {
    __spec_runner__.context(name, block)
}

fn it(description, block) {
    __spec_runner__.it(description, block)
}

fn xit(description, block) {
    __spec_runner__.xit(description, block)
}

fn pending(description) {
    __spec_runner__.pending(description)
}

fn before_each(hook) {
    __spec_runner__.before_each(hook)
}

fn after_each(hook) {
    __spec_runner__.after_each(hook)
}

# expect and assert are already exported from gspec module
fn expect(value) {
    return gspec.expect(value)
}

fn assert(result) {
    return gspec.assert(result)
}
"#;

    execute_source(setup_code, executor)
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
