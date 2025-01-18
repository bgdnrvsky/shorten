//! https://crates.io/crates/tico

use super::{Decorator, PathBuf};

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Component;

pub struct Tico {
    path: PathBuf,
}

impl Tico {
    pub fn new(path: impl Decorator) -> Self {
        Self {
            path: path.decorate(),
        }
    }
}

impl Decorator for Tico {
    fn decorate(&self) -> PathBuf {
        let mut components = self.path.components().collect::<Vec<_>>();

        for component in components.iter_mut().rev().skip(1).rev() {
            if let Component::Normal(osstr) = component {
                let cow = osstr.to_string_lossy();
                let mut chars = cow.chars().peekable();
                let take_length = if let Some('.') = chars.peek() { 2 } else { 1 };
                let prefix = chars.take(take_length);
                *osstr = OsStr::from_bytes(&osstr.as_bytes()[..prefix.map(char::len_utf8).sum()]);
            }
        }

        PathBuf::from_iter(components)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::decorators::Decorator;

    use super::Tico;

    #[test]
    fn just_root() {
        let root = PathBuf::from("/");

        assert_eq!(Tico::new(&root).decorate(), root);
    }

    #[test]
    fn just_home() {
        let home = PathBuf::from("~");

        assert_eq!(Tico::new(&home).decorate(), home);
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Tico::new(PathBuf::from("/home/hugopeixoto/work/personal/tico")).decorate(),
            PathBuf::from("/h/h/w/p/tico")
        );
        assert_eq!(
            Tico::new(PathBuf::from("~/work/personal/tico")).decorate(),
            PathBuf::from("~/w/p/tico")
        );
        assert_eq!(
            Tico::new(PathBuf::from("~/work/ééé/tico")).decorate(),
            PathBuf::from("~/w/é/tico")
        );
    }

    #[test]
    fn with_dot_prefixed() {
        assert_eq!(
            Tico::new(PathBuf::from("/home/feodot/.local/share/.hidden")).decorate(),
            PathBuf::from("/h/f/.l/s/.hidden")
        );
    }
}
