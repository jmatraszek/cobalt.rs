#![deny(warnings)]

extern crate liquid;
extern crate markdown;
extern crate walkdir;
extern crate crossbeam;
extern crate chrono;
extern crate yaml_rust;

pub use cobalt::build;
pub use error::Error;

// modules
mod cobalt;
pub mod error;
mod document;
