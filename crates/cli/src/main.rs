//! Welcome to `Termify` 🤠
use std::process::ExitCode;
mod cli;

fn main() -> ExitCode {
    match cli::Cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("termify: {err:#}");
            ExitCode::FAILURE
        }
    }
}
