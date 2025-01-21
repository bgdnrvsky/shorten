use super::{Decorator, PathBuf};

pub struct Canicolizer<D> {
    wrapee: D,
}

impl<D> Canicolizer<D> {
    pub fn new(wrapee: D) -> Self {
        Self { wrapee }
    }
}

impl<D: Decorator> Decorator for Canicolizer<D> {
    fn decorate(&self) -> PathBuf {
        let path = self.wrapee.decorate();

        match path.canonicalize() {
            Ok(decorated) => decorated,
            Err(e) => {
                eprintln!("Couldn't canonicalize: {e}");
                path.clone()
            }
        }
    }
}
