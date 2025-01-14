pub(crate) use std::path::Path;
pub(crate) use std::path::PathBuf;

mod canicolizer;
mod home_stripper;
mod shortener;
mod tico;

pub use canicolizer::Canicolizer;
pub use home_stripper::HomeStripper;
pub use shortener::Shortener;
pub use tico::Tico;

pub(crate) trait Decorator {
    fn decorate(&self) -> PathBuf;
}

impl<P: AsRef<Path>> Decorator for P {
    fn decorate(&self) -> PathBuf {
        self.as_ref().to_owned()
    }
}

impl Decorator for Box<dyn Decorator> {
    fn decorate(&self) -> PathBuf {
        self.as_ref().decorate()
    }
}
