//! Welcome to `Termify` 🤠
use std::process::ExitCode;
use termify::cli::Cli;

fn main() -> ExitCode {
    match Cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("termify: {err:#}");
            ExitCode::FAILURE
        }
    }
}
