use std::path::PathBuf;

use clap::ArgAction;
use clap::Args;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Shorten {
    /// The path to shorten
    pub path: PathBuf,

    /// Canonicalize the path
    #[arg(short, long, action=ArgAction::SetTrue)]
    pub canonicalize: bool,

    /// Replace prefix home directory with ~ if possible
    #[arg(long, action=ArgAction::SetTrue)]
    pub home: bool,

    /// Disable `tico` strategy: ~/work/personal/tico -> ~/w/p/tico
    #[arg(short, long, action=ArgAction::SetFalse)]
    pub tico: bool,

    /// Disable path `shortening` this/is/a/very/long/path/ -> this/is/.../long/path
    #[arg(short, long, action=ArgAction::SetFalse)]
    pub shorten: bool,

    #[clap(flatten)]
    pub path_shortener: PathShortenerOptions,
}

#[derive(Args, Default)]
pub struct PathShortenerOptions {
    /// Replacement patter
    #[arg(long, default_value = "...")]
    pub replacement: String,
    /// How many components to leave on the left side
    #[arg(long, default_value_t = 2)]
    pub left: usize,
    /// How many components to leave on the right side
    #[arg(long, default_value_t = 2)]
    pub right: usize,
}
