use std::{borrow::Cow, path::Path};

use super::Decorator;

pub struct Canicolizer<D> {
    wrapee: D,
}

impl<D> Canicolizer<D> {
    pub fn new(wrapee: D) -> Self {
        Self { wrapee }
    }
}

impl<D: Decorator> Decorator for Canicolizer<D> {
    fn decorate(&self) -> Cow<Path> {
        let path = self.wrapee.decorate();

        match path.canonicalize() {
            Ok(decorated) => Cow::Owned(decorated),
            Err(e) => {
                eprintln!("Couldn't canonicalize: {e}");
                path
            }
        }
    }
}
