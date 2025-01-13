use super::{Decorator, PathBuf};

pub struct Canicolizer {
    path: PathBuf,
}

impl Canicolizer {
    pub fn new(path: impl Decorator) -> Self {
        Self {
            path: path.decorate(),
        }
    }
}

impl Decorator for Canicolizer {
    fn decorate(&self) -> PathBuf {
        match self.path.canonicalize() {
            Ok(decorated) => decorated,
            Err(e) => {
                eprintln!("Couldn't canonicalize: {e}");
                self.path.clone()
            }
        }
    }
}
