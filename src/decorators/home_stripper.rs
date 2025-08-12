use std::{borrow::Cow, path::Path};

use super::{Decorator, PathBuf};

pub struct HomeStripper<D> {
    wrapee: D,
}

impl<D> HomeStripper<D> {
    pub fn new(wrapee: D) -> Self {
        Self { wrapee }
    }
}

impl<D: Decorator> Decorator for HomeStripper<D> {
    fn decorate(&self) -> Cow<Path> {
        let path = self.wrapee.decorate();

        if let Some(home_dir) = dirs::home_dir() {
            match path.strip_prefix(home_dir) {
                Ok(suffix) => Cow::Owned(PathBuf::from("~").join(suffix)),
                Err(e) => {
                    eprintln!("Couldn't strip prefix: {e}");
                    path
                }
            }
        } else {
            eprintln!("Couldn't get home directory");
            path
        }
    }
}
