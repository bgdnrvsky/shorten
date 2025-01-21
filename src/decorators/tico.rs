//! https://crates.io/crates/tico

use super::{Decorator, PathBuf};

use std::ffi::OsStr;
use std::path::Component;

pub struct Tico<D> {
    wrapee: D,
}

impl<D> Tico<D> {
    pub fn new(wrapee: D) -> Self {
        Self { wrapee }
    }
}

impl<D: Decorator> Decorator for Tico<D> {
    fn decorate(&self) -> PathBuf {
        let path = self.wrapee.decorate();
        let mut components = path.components().collect::<Vec<_>>();

        for component in components.iter_mut().rev().skip(1).rev() {
            if let Component::Normal(osstr) = component {
                let cow = osstr.to_string_lossy();
                let mut chars = cow.chars().peekable();
                let take_length = if let Some('.') = chars.peek() { 2 } else { 1 };
                let prefix = chars.take(take_length);
                #[cfg(unix)]
                {
                    use std::os::unix::ffi::OsStrExt;
                    *osstr =
                        OsStr::from_bytes(&osstr.as_bytes()[..prefix.map(char::len_utf8).sum()]);
                }
                #[cfg(windows)]
                {
                    unsafe {
                        *osstr = OsStr::from_encoded_bytes_unchecked(
                            &osstr.as_encoded_bytes()[..prefix.map(char::len_utf8).sum()],
                        );
                    }
                }
            }
        }

        PathBuf::from_iter(components)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::decorators::{Decorator, Plain};

    use super::Tico;

    #[test]
    fn just_root() {
        let root = Plain::new(PathBuf::from("/"));

        assert_eq!(Tico::new(&root).decorate(), root.path());
    }

    #[test]
    fn just_home() {
        let home = Plain::new(PathBuf::from("~"));

        assert_eq!(Tico::new(&home).decorate(), home.path());
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Tico::new(Plain::new(PathBuf::from(
                "/home/hugopeixoto/work/personal/tico"
            )))
            .decorate(),
            PathBuf::from("/h/h/w/p/tico")
        );
        assert_eq!(
            Tico::new(Plain::new(PathBuf::from("~/work/personal/tico"))).decorate(),
            PathBuf::from("~/w/p/tico")
        );
        assert_eq!(
            Tico::new(Plain::new(PathBuf::from("~/work/ééé/tico"))).decorate(),
            PathBuf::from("~/w/é/tico")
        );
    }

    #[test]
    fn with_dot_prefixed() {
        assert_eq!(
            Tico::new(Plain::new(PathBuf::from(
                "/home/feodot/.local/share/.hidden"
            )))
            .decorate(),
            PathBuf::from("/h/f/.l/s/.hidden")
        );
    }
}
