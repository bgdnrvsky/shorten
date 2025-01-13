use super::{Decorator, PathBuf};

pub struct HomeStripper {
    path: PathBuf,
}

impl HomeStripper {
    pub fn new(path: impl Decorator) -> Self {
        Self {
            path: path.decorate(),
        }
    }
}

impl Decorator for HomeStripper {
    fn decorate(&self) -> PathBuf {
        if let Some(home_dir) = dirs::home_dir() {
            match self.path.strip_prefix(home_dir) {
                Ok(suffix) => PathBuf::from("~").join(suffix),
                Err(e) => {
                    eprintln!("Couldn't strip: {e}");
                    self.path.clone()
                }
            }
        } else {
            eprintln!("Couldn't get home directory");
            self.path.clone()
        }
    }
}
