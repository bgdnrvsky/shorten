pub mod cli;
mod decorators;

use cli::Shorten;

use clap::Parser;
use decorators::*;

fn main() {
    let shorten = Shorten::parse();
    let mut path: Box<dyn Decorator> = Box::new(Plain::new(shorten.path));

    if shorten.canonicalize {
        path = Box::new(Canicolizer::new(path));
    }

    if shorten.home {
        path = Box::new(HomeStripper::new(path));
    }

    if shorten.tico {
        path = Box::new(Tico::new(path));
    }

    if shorten.shorten {
        path = Box::new(Shortener::new(path, shorten.path_shortener));
    }

    print!("{}", path.decorate().display());
}
