//! A test program decomposing a quantum state into an eigenbasis.
#![deny(missing_docs)]

pub mod framework;
pub(crate) mod frontend;

fn main() {
    frontend::run();
}
