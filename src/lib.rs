//! Ergonomic use of environment variables
//!
//! Allows reading and unpacking the value of an environment variable into a desired type, abstracting away the error handling
//! and parsing logic to provide enhanced readability.

pub mod env_var;

#[cfg(test)]
mod tests;
