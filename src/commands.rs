use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use crate::train;

#[derive(Subcommand)]
pub enum Commands {
    Train {
        #[clap(short, long, default_value = "train")]
        folder: PathBuf,
    },
}

impl Commands {
    pub fn exec(&self) -> Result<()> {
        match &self {
            Self::Train { folder } => {
                train::start::train(folder.to_owned())?;
            }
        }

        Ok(())
    }
}
