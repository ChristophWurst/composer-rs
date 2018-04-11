#[macro_use]
extern crate failure;

use std::path::PathBuf;
use std::process::Command;

use failure::Error;

pub struct Composer {
    path: PathBuf,
}

impl Composer {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Composer {
            path: path.into().join("composer.json"),
        }
    }

    pub fn is_available(&self) -> bool {
        println!("{:?}", self.path);
        self.path.exists()
    }

    fn ensure_available(&self) -> Result<(), Error> {
        if !self.is_available() {
            format_err!("no composer executable found");
        }
        Ok(())
    }

    pub fn install(&self) -> Result<(), Error> {
        self.ensure_available()?;

        Command::new("composer").arg("install").output()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_not_available_in_the_root_dir() {
        let composer = Composer::new(".");

        assert!(!composer.is_available());
    }

    #[test]
    fn it_is_available_in_the_examples_dir() {
        let composer = Composer::new("examples");

        assert!(composer.is_available());
    }

    #[test]
    fn it_installs_deps() {
        let composer = Composer::new("examples");

        composer.install().unwrap();
    }
}
