// This can probably be removed soon. See:
// https://github.com/steveklabnik/rustdoc/issues/96
#![allow(unused_doc_comment)]

//! The error type for this crate.

error_chain! {
    errors {
        /// Error opening the I2C device.
        FailedToOpenDevice {}
    }

    foreign_links {
        I2C(::i2cdev::linux::LinuxI2CError) #[doc = "An error from an I2C device."];
        IO(::std::io::Error) #[doc = "An IO error."];
    }
}
