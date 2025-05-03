use bfcompiler::{
    compiler::build_bf, interpreter::interpret_bf, repl::repl_bf, utils::read_bf_file,
};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "bfcompiler",
    about = "A Brainfuck interpreter and compiler",
    version = "0.1.0",
    long_about = "bfcompiler is a tool to interpret or compile Brainfuck code. Use subcommands to run a Brainfuck program, compile it to Rust, or start an interactive REPL."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a Brainfuck program directly
    Run {
        /// Path to the Brainfuck source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Compile a Brainfuck program to Rust code
    Build {
        /// Path to the Brainfuck source file
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Save the generated Rust code
        #[arg(long, short)]
        save: bool,
    },
    /// Start an interactive Brainfuck REPL
    Repl,
}

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Commands::Run { file } => interpret_bf(&read_bf_file(&file)?)?,
        Commands::Build { file, save } => build_bf(file, save)?,
        Commands::Repl => repl_bf()?,
    }
    Ok(())
}
