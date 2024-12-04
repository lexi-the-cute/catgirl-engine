use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Parser, Debug, Clone, PartialEq, PartialOrd)]
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
