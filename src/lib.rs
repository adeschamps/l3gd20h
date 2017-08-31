#![deny(missing_docs)]

//! Interface to the L3GD20H digital gyroscope.
//!
//! - [Datasheet](https://cdn-shop.adafruit.com/datasheets/L3GD20H.pdf)


// External crates

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate error_chain;

extern crate i2cdev;


// Exports

mod errors;
pub use errors::{Error, ErrorKind, Result, ResultExt};

pub mod registers;

pub mod gyroscope;
pub use gyroscope::Gyroscope;
