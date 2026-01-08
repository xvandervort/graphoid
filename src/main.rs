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

    let mut total_passed = 0;
    let mut total_failed = 0;
    let mut total_skipped = 0;
    let mut failed_files: Vec<String> = Vec::new();

    for spec_file in &spec_files {
        match run_spec_file(spec_file) {
            Ok((passed, failed, skipped)) => {
                total_passed += passed;
                total_failed += failed;
                total_skipped += skipped;
                if failed > 0 {
                    failed_files.push(spec_file.display().to_string());
                }
            }
            Err(e) => {
                eprintln!("\nError in {}: {}", spec_file.display(), e);
                total_failed += 1;
                failed_files.push(spec_file.display().to_string());
            }
        }
    }

    // Print final summary
    println!("\n{}", "=".repeat(60));
    println!("FINAL SUMMARY");
    println!("{}", "=".repeat(60));

    let total = total_passed + total_failed + total_skipped;
    println!("{} passed, {} failed, {} skipped, {} total",
             total_passed, total_failed, total_skipped, total);

    if !failed_files.is_empty() {
        println!("\nFailed files:");
        for f in &failed_files {
            println!("  - {}", f);
        }
    }

    if total_failed > 0 {
        println!("\nSOME TESTS FAILED");
        std::process::exit(1);
    } else if total_skipped > 0 {
        println!("\nALL TESTS PASSED (some skipped)");
    } else {
        println!("\nALL TESTS PASSED");
    }
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

fn run_spec_file(path: &Path) -> Result<(i64, i64, i64), String> {
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let mut executor = Executor::new();

    // Set current file for module resolution
    let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    executor.set_current_file(Some(abs_path));

    // Inject gspec DSL functions into global scope
    inject_spec_dsl(&mut executor)?;

    // Execute the spec file
    execute_source(&source, &mut executor)?;

    // Get results from the runner
    let summary = get_spec_summary(&mut executor)?;

    Ok(summary)
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

fn get_spec_summary(executor: &mut Executor) -> Result<(i64, i64, i64), String> {
    // Call __spec_runner__.summary() to get results
    let summary_code = "__spec_runner__.summary()";

    let mut lexer = Lexer::new(summary_code);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    if let Some(stmt) = program.statements.first() {
        if let graphoid::ast::Stmt::Expression { expr, .. } = stmt {
            let value = executor.eval_expr(expr)
                .map_err(|e| format!("Runtime error: {}", e))?;

            // Extract passed, failed, skipped from the map
            if let Value { kind: graphoid::values::ValueKind::Map(map), .. } = value {
                let passed = extract_num_from_hash(&map, "passed").unwrap_or(0);
                let failed = extract_num_from_hash(&map, "failed").unwrap_or(0);
                let skipped = extract_num_from_hash(&map, "skipped").unwrap_or(0);
                return Ok((passed, failed, skipped));
            }
        }
    }

    Err("Failed to get spec summary".to_string())
}

fn extract_num_from_hash(hash: &graphoid::values::Hash, key: &str) -> Option<i64> {
    hash.get(key).and_then(|v| {
        if let graphoid::values::ValueKind::Number(n) = &v.kind {
            Some(*n as i64)
        } else {
            None
        }
    })
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
