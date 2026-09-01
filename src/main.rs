use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

mod analyzer;
mod converter;
mod optimizer;
mod generator;

#[derive(Parser)]
#[command(name = "Catalyst")]
#[command(about = "Python-to-Rust conversion framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze Python code
    Analyze {
        /// Path to Python file or directory
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    /// Convert Python to Rust
    Convert {
        /// Input Python file
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Output Rust file
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },
    /// Optimize Rust code
    Optimize {
        /// Path to Rust file
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    /// Generate benchmarks
    Benchmark {
        /// Python file to benchmark
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path } => {
            info!("Analyzing: {:?}", path);
            analyzer::analyze(&path);
        }
        Commands::Convert { input, output } => {
            info!("Converting: {:?}", input);
            let out = output.unwrap_or_else(|| {
                input.with_extension("rs")
            });
            converter::convert(&input, &out);
        }
        Commands::Optimize { path } => {
            info!("Optimizing: {:?}", path);
            optimizer::optimize(&path);
        }
        Commands::Benchmark { file } => {
            info!("Generating benchmark for: {:?}", file);
            generator::generate_benchmark(&file);
        }
    }
}
