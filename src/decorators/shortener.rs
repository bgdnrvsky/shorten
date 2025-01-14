use super::super::cli::PathShortenerOptions;
use super::{Decorator, PathBuf};

pub struct Shortener {
    path: PathBuf,
    parameters: PathShortenerOptions,
}

impl Shortener {
    pub fn new(path: impl Decorator, parameters: PathShortenerOptions) -> Self {
        Self {
            path: path.decorate(),
            parameters,
        }
    }
}

impl Decorator for Shortener {
    fn decorate(&self) -> PathBuf {
        let components = self.path.components().collect::<Vec<_>>();

        if components.len() <= self.parameters.left + self.parameters.right {
            return self.path.clone();
        }

        let left = PathBuf::from_iter(components.iter().take(self.parameters.left));
        let right = PathBuf::from_iter(components.iter().rev().take(self.parameters.right).rev());

        left.join(self.parameters.replacement.clone()).join(right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let parameters = PathShortenerOptions {
            replacement: "...".to_string(),
            left: 2,
            right: 2,
        };
        let path = PathBuf::from("this/is/a/very/long/path/");
        let shortener = Shortener::new(path, parameters);

        assert_eq!(shortener.decorate(), PathBuf::from("this/is/.../long/path"));
    }

    /// Length of the input path is exactly left + right
    #[test]
    fn left_plus_right() {
        let parameters = PathShortenerOptions {
            replacement: "...".to_string(),
            left: 2,
            right: 2,
        };
        let path = PathBuf::from("a/b/c/d");
        let shortener = Shortener::new(&path, parameters);

        assert_eq!(shortener.decorate(), path);
    }

    #[test]
    fn too_short() {
        let parameters = PathShortenerOptions {
            replacement: "...".to_string(),
            left: 2,
            right: 2,
        };
        let path = PathBuf::from("a/b/c");
        let shortener = Shortener::new(&path, parameters);

        assert_eq!(shortener.decorate(), path);
    }
}
