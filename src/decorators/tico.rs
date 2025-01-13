//! https://crates.io/crates/tico

use super::{Decorator, PathBuf};

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, MAIN_SEPARATOR};

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

        let ends_with_separator = self.path.to_string_lossy().ends_with(MAIN_SEPARATOR);

        if ends_with_separator {
            components.push(Component::RootDir);
        }

        for component in components.iter_mut().rev().skip(1).rev() {
            if let Component::Normal(osstr) = component {
                let first: char = osstr.to_string_lossy().chars().next().unwrap();
                *osstr = OsStr::from_bytes(&osstr.as_bytes()[..first.len_utf8()]);
            }
        }

        if ends_with_separator {
            components.pop();
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
            Tico::new(PathBuf::from("~/work/personal/tico/")).decorate(),
            PathBuf::from("~/w/p/t/")
        );
        assert_eq!(
            Tico::new(PathBuf::from("~/work/ééé/tico")).decorate(),
            PathBuf::from("~/w/é/tico")
        );
    }
}
