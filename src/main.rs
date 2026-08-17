//! Welcome to `Termify` 🤠
use std::process::ExitCode;

fn main() -> ExitCode {
    match termify::cli::Cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("termify: {err:#}");
            ExitCode::FAILURE
        }
    }
}
