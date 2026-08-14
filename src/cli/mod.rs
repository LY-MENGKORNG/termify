//! The command line arguments for the app.

pub mod constant;

use anyhow::{Context, Result};
use clap::Parser;

use crate::{
    cli::constant::BANNER,
    config::{Conf, err::ConfErr, paths::Paths},
    services::logger::{Logger, constant::FILTER_ENV_LOG},
};

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
            println!("configuration  {}", paths.config_file().display());
            println!("themes         {}", paths.themes_dir().display());
            println!("saved state    {}", paths.state_file().display());
            println!("session token  {}", paths.token_file().display());
            println!("audio token    {}", paths.streaming_token_file().display());
            println!("audio cache    {}", paths.librespot_dir().display());
            println!("log            {}", paths.log_file().display());
            println!();
            println!(
                "Set {} to change the log level, e.g. termify=debug",
                FILTER_ENV_LOG
            );
            return Ok(());
        }

        Logger::init(&paths.log_file()).context("could not start logging")?;

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

        let _conf = match Conf::load(&paths) {
            Ok(conf) => conf,
            Err(err @ ConfErr::CreatedTemplate { .. }) => {
                println!("{err}");
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        };

        // TODO: Create tokio runtime new multi thread and run on it.

        Ok(())
    }

    fn logout(_paths: &Paths) -> Result<bool> {
        // TODO: Forgets all necessary credentails
        Ok(true)
    }
}
