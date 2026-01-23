mod habit;
mod storage;
mod validation;
mod cli;
mod tui;
pub mod stats;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--tui" {
        tui::run()?;
    } else {
        cli::run();
    }
    
    Ok(())
}