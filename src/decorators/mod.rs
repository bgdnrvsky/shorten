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

#[derive(Debug)]
pub struct Plain {
    path: PathBuf,
}

impl Plain {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[cfg(test)]
    pub fn path(self) -> PathBuf {
        self.path
    }
}

impl Decorator for Plain {
    fn decorate(&self) -> PathBuf {
        self.path.clone()
    }
}

impl<D: Decorator + ?Sized> Decorator for Box<D> {
    #[inline]
    fn decorate(&self) -> PathBuf {
        D::decorate(self)
    }
}

impl<D: Decorator + ?Sized> Decorator for &'_ D {
    #[inline]
    fn decorate(&self) -> PathBuf {
        D::decorate(*self)
    }
}
