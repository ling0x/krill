// Compiler CLI
use anyhow::Result;

mod ast;
mod bytecode;
mod interpreter;
mod typechecker;

// LALRPOP-generated parser module wrapper in src/grammar.rs
mod grammar;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: agentc <source.agent>");
        std::process::exit(1);
    }

    let source = std::fs::read_to_string(&args[1])?;

    // Parse
    let parser = grammar::ProgramParser::new();
    let program = parser
        .parse(&source)
        .map_err(|e| anyhow::anyhow!("Parse error: {:?}", e))?;

    println!("✓ Parsed successfully");

    // Type check
    typechecker::typecheck(&program)?;
    println!("✓ Type checked successfully");

    // Compile to bytecode
    let bytecode_program = bytecode::compile(&program)?;
    println!("✓ Compiled to bytecode");

    // Execute
    println!("\nExecuting...\n");
    interpreter::execute(bytecode_program).await?;

    Ok(())
}
