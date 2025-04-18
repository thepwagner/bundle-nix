use std::io;
use std::path::Path;

extern crate packageurl;

pub trait Dependency {
    fn package_url(&self) -> packageurl::PackageUrl;
}

pub struct Dependencies<T: Dependency> {
    // pub development: Vec<T>,
    pub runtime: Vec<T>,
}

impl<T: Dependency> Dependencies<T> {
    pub fn new() -> Self {
        Dependencies { runtime: Vec::new() }
    }
}

pub type ParseFn<T: Dependency> = fn(path: &Path) -> Result<Dependencies<T>, io::Error>;

pub type DerivationFn<T: Dependency> = fn(dependencies: &Dependencies<T>) -> String;