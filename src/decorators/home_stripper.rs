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
    fn decorate(&self) -> PathBuf {
        let path = self.wrapee.decorate();

        if let Some(home_dir) = dirs::home_dir() {
            match path.strip_prefix(home_dir) {
                Ok(suffix) => PathBuf::from("~").join(suffix),
                Err(_) => path.clone(),
            }
        } else {
            eprintln!("Couldn't get home directory");
            path.clone()
        }
    }
}
