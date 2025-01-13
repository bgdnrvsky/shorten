use std::path::PathBuf;

use clap::ArgAction;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Shorten {
    /// The path to shorten
    pub path: PathBuf,

    /// Canonicalize the path
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub canonicalize: bool,

    // Replace prefix home directory with ~ if possible
    #[arg(long, action=ArgAction::SetTrue)]
    pub home: bool,

    /// Disable `tico` strategy: ~/work/personal/tico -> ~/w/p/tico
    #[arg(short, long, action=ArgAction::SetFalse)]
    pub tico: bool,
}
