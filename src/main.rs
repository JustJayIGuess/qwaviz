//! A test program decomposing a quantum state into an eigenbasis.
// #![deny(missing_docs)]

mod frontend;
pub mod framework;

fn main() {
    frontend::run();
}