use std::path::PathBuf;

use clap::Parser;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Parser, Debug, Clone, PartialEq, PartialOrd, Default, Hash, Eq, Ord)]
#[command(
    author,
    about="A game engine for cool moddability and procedurally generated data",
    long_about = None
)]
/// List of possible command line arguments
pub struct Args {
    /// Start the engine in dedicated server mode
    #[arg(short, long, default_value_t = false)]
    pub server: bool,

    /// Display version and copyright info
    #[arg(short, long, default_value_t = false)]
    pub version: bool,

    /// Set custom resources path
    #[arg(short, long, default_value = "resources")]
    pub resources: PathBuf,

    /// Print all environment variables
    #[arg(long, default_value_t = false)]
    pub print_environment_variables: bool,
}
