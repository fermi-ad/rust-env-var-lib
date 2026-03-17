//! Env Var Module
//!
//! This module provides the funtions and structs for easily interacting with the environment.
//! Wrapping details in this module lets consuming code more clearly disambiguate from [`mod@std::env`].

use std::{
    env::{self, VarError},
    fmt::{Debug, Formatter, Result as FmtResult},
    str::FromStr,
};
use tracing::warn;

/// An intermediary object to handle converting the [`Result`] from [`std::env::var`] into a concrete type.    
pub struct EnvVal {
    result: Result<String, VarError>,
    var_name: String,
}
impl EnvVal {
    /// Unwraps the result of reading the specified environment variable. In the event there was an error,
    /// logs the error and returns the provided default value.
    pub fn or<T: FromStr + Debug>(self, default: T) -> T
    where
        T::Err: Debug,
    {
        match self.try_parsing() {
            Ok(val) => val,
            Err(err) => {
                warn!("{:?}: {}. Using default: {:?}", err, self.var_name, default);
                default
            }
        }
    }

    /// Unwraps the result of reading the specified environment variable. In the event there was an error,
    /// logs the error and generates the default value using the provided function.
    pub fn or_else<T: FromStr + Debug>(self, default_fn: impl Fn() -> T) -> T
    where
        T::Err: Debug,
    {
        match self.try_parsing() {
            Ok(val) => val,
            Err(err) => {
                let generated_default = default_fn();
                warn!(
                    "{:?}: {}. Using default: {:?}",
                    err, self.var_name, generated_default
                );
                generated_default
            }
        }
    }

    /// Unwraps the result of reading the specified environment variable into an instance of [`Some`]. In the
    /// event there was an error, logs the error and returns [`None`].
    pub fn to_option<T: FromStr + Debug>(self) -> Option<T>
    where
        T::Err: Debug,
    {
        match self.try_parsing() {
            Ok(val) => Some(val),
            Err(err) => {
                warn!("{:?}: {}. Returning None.", err, self.var_name);
                None
            }
        }
    }

    fn try_parsing<T: FromStr + Debug>(&self) -> Result<T, ParseErr<'_, T>>
    where
        T::Err: Debug,
    {
        match self.result.as_ref() {
            Ok(val) => val.parse().map_err(|err| ParseErr::Parsing(err)),
            Err(err) => Err(ParseErr::Env(err)),
        }
    }
}

/// Attempts to read the provided environment variable and parse to the desired type.
///
/// # Panics
/// Panics if either the specified variable is not set or if its value cannot be parsed
/// to the needed type.
pub fn expect<T: FromStr + Debug>(var: &str) -> T
where
    T::Err: Debug,
{
    env::var(var)
        .map_err(|err| ParseErr::<T>::EnvOwned(err))
        .and_then(|val| val.parse().map_err(|err| ParseErr::Parsing(err)))
        .unwrap()
}

/// Reads the provided environment variable, generating an instance of [`EnvVal`] to handle
/// translating the result to a useful value.
pub fn get(var: &str) -> EnvVal {
    EnvVal {
        var_name: var.to_owned(),
        result: env::var(var),
    }
}

enum ParseErr<'p, T: FromStr + Debug>
where
    T::Err: Debug,
{
    Env(&'p VarError),
    EnvOwned(VarError),
    Parsing(T::Err),
}
impl<'p, T: FromStr + Debug> Debug for ParseErr<'p, T>
where
    T::Err: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Env(var_err) => write!(f, "{var_err:?}"),
            Self::EnvOwned(var_err) => write!(f, "{var_err:?}"),
            Self::Parsing(parse_err) => write!(f, "{parse_err:?}"),
        }
    }
}
