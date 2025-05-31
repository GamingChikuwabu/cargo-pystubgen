mod cli;
use clap::Parser;
use cli::{Cli, Commands};
mod stubgen;
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { project_dir, output_dir, debug } => {
            println!("Generating Python stubs...");
            if let Some(dir) = project_dir.clone() {
                println!("Project directory: {}", dir.display());
            }
            if let Some(dir) = output_dir.clone() {
                println!("Output directory: {}", dir.display());
            }
            stubgen::generate_stubs(debug);
        }
    }
}