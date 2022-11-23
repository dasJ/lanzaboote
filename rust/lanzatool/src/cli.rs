use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::install;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        public_key: PathBuf,
        bootspec: PathBuf,
    },
}

impl Cli {
    pub fn call(self) -> Result<()> {
        self.commands.call()
    }
}

impl Commands {
    pub fn call(self) -> Result<()> {
        match self {
            Commands::Install {
                public_key,
                bootspec,
            } => install(&public_key, &bootspec),
        }
    }
}

fn install(public_key: &Path, bootspec: &Path) -> Result<()> {
    let lanzaboote_stub = std::env::var("LANZABOOTE_STUB")?;
    let initrd_stub = std::env::var("LANZABOOTE_INITRD_STUB")?;
    install::install(
        public_key,
        bootspec,
        Path::new(&lanzaboote_stub),
        Path::new(&initrd_stub),
    )
}
