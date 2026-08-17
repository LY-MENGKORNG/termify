//! The command line arguments for the app.
use anyhow::{Context, Result};
use clap::Parser;
use termify_core::app;
use termify_core::config::{self, ConfigError, Paths};
use termify_core::constant::BANNER;
use termify_core::service::logger;

#[derive(Debug, Parser)]
#[command(name = "termify", version, about, long_about = None)]
pub struct Cli {
    /// print configuration/logs paths to the std output.
    #[arg(long)]
    pub paths: bool,

    /// logout the authenticated Spotify acc.
    #[arg(long)]
    pub logout: bool,
}

impl Cli {
    pub fn run() -> Result<()> {
        let cli = Self::parse();
        let paths = Paths::resolve()?;

        println!("{BANNER}");

        if cli.paths {
            paths.print_paths();
            return Ok(());
        }

        logger::init(&paths.log_file()).context("could not start logging")?;

        if cli.logout {
            let logged_out = Self::logout(&paths)?;

            println!(
                "{}",
                if logged_out {
                    "Signed out."
                } else {
                    "No cached session to remove."
                }
            );

            return Ok(());
        }

        let conf = match config::load(&paths) {
            Ok(conf) => conf,
            Err(err @ ConfigError::CreatedTemplate { .. }) => {
                println!("{err}");
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        };

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .context("could not start the async runtime")?;

        runtime.block_on(app::init(conf, &paths))
    }

    fn logout(_paths: &Paths) -> Result<bool> {
        // TODO: Forgets all necessary credentails
        Ok(true)
    }
}
