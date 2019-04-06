#[macro_use]

extern crate lazy_static;
extern crate regex;

// This library only exists to allow us to benchmark with criterion.
// There might be a better way to do this.

pub mod apps;
pub mod files;
pub mod filter;
