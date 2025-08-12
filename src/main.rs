pub mod cli;
mod decorators;

use cli::Shorten;

use clap::Parser;
use decorators::*;

fn main() {
    let shorten = Shorten::parse();
    let mut decorator: Box<dyn Decorator> = Box::new(Plain::new(shorten.path));

    if shorten.canonicalize {
        decorator = Box::new(Canicolizer::new(decorator));
    }

    if shorten.home {
        decorator = Box::new(HomeStripper::new(decorator));
    }

    if shorten.tico {
        decorator = Box::new(Tico::new(decorator));
    }

    if shorten.shorten {
        decorator = Box::new(Shortener::new(decorator, shorten.path_shortener));
    }

    print!("{}", decorator.decorate().display());
}
