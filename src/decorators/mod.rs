use std::borrow::Cow;
pub(crate) use std::path::{Path, PathBuf};

mod canicolizer;
mod home_stripper;
mod shortener;
mod tico;

pub use canicolizer::Canicolizer;
pub use home_stripper::HomeStripper;
pub use shortener::Shortener;
pub use tico::Tico;

pub(crate) trait Decorator {
    fn decorate(&self) -> Cow<Path>;
}

#[derive(Debug)]
pub struct Plain {
    path: PathBuf,
}

impl Plain {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[cfg(test)]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Decorator for Plain {
    fn decorate(&self) -> Cow<Path> {
        Cow::Borrowed(&self.path)
    }
}

impl<D: Decorator + ?Sized> Decorator for Box<D> {
    #[inline]
    fn decorate(&self) -> Cow<Path> {
        D::decorate(self)
    }
}

impl<D: Decorator + ?Sized> Decorator for &'_ D {
    #[inline]
    fn decorate(&self) -> Cow<Path> {
        D::decorate(*self)
    }
}
