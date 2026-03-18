// Forbid unwrap in production code
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// Allow unwrap in all tests (including integration tests)
#![cfg_attr(test, allow(clippy::unwrap_used))]
#![cfg_attr(test, allow(clippy::expect_used))]

pub mod cli;
pub mod config;
pub mod constant;
pub mod error;
pub mod hook;
pub mod parser;
pub mod util;
pub mod validator;
