//! Ergonomic use of environment variables
//!
//! Allows reading and unpacking the value of an environment variable into a desired type, abstracting away the error handling
//! and parsing logic to provide enhanced readability.

/// This module provides the funtions and structs for easily interacting with the environment.
/// Wrapping details in this module lets consuming code more clearly disambiguate from [`mod@std::env`].
pub mod env_var {
    use std::{
        env::{self, VarError},
        fmt::Display,
        str::FromStr,
    };
    use tracing::{error, warn};
    /// An intermediary object to handle converting the [`Result`] from [`std::env::var`] into a concrete type.    
    pub struct EnvVal {
        var_name: String,
        result: Result<String, VarError>,
    }
    impl EnvVal {
        /// Unwraps the result of reading the specified environment variable. In the event there was an error,
        /// logs the error and returns the provided default value.
        pub fn or<T: FromStr + Display>(self, default: T) -> T {
            match self.result {
                Ok(val) => match val.parse::<T>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        error!(
                            "Could not read the value for {}. Using default: {}",
                            self.var_name, default
                        );
                        default
                    }
                },
                Err(err) => {
                    warn!("{}: {}. Using default: {}", err, self.var_name, default);
                    default
                }
            }
        }

        /// Unwraps the result of reading the specified environment variable. In the event there was an error,
        /// logs the error and generates the default value using the provided function.
        pub fn or_else<T: FromStr + Display>(self, default_fn: impl Fn() -> T) -> T {
            match self.result {
                Ok(val) => match val.parse::<T>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        let generated_default = default_fn();
                        error!(
                            "Could not read the value for {}. Using default: {}",
                            self.var_name, generated_default
                        );
                        generated_default
                    }
                },
                Err(err) => {
                    let generated_default = default_fn();
                    warn!(
                        "{}: {}. Using default: {}",
                        err, self.var_name, generated_default
                    );
                    generated_default
                }
            }
        }

        /// Unwraps the result of reading the specified environment variable into an instance of [`Some`]. In the
        /// event there was an error, logs the error and returns [`None`].
        pub fn to_option<T: FromStr + Display>(self) -> Option<T> {
            match self.result {
                Ok(val) => match val.parse::<T>() {
                    Ok(parsed) => Some(parsed),
                    Err(_) => {
                        error!(
                            "Could not parse the value for {}. Returning None.",
                            self.var_name
                        );
                        None
                    }
                },
                Err(err) => {
                    warn!("{}: {}. Returning None.", err, self.var_name);
                    None
                }
            }
        }
    }

    /// Reads the provided environment variable, generating an instance of [`EnvVal`] to handle
    /// translating the result to a useful value.
    pub fn get(var: &str) -> EnvVal {
        EnvVal {
            var_name: var.to_owned(),
            result: env::var(var),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        const TEST_VAL: &str = "test";
        const FAIL_VAL: &str = "fail";

        #[test]
        fn get_var() {
            const KEY: &str = "get_var";
            unsafe {
                env::set_var(KEY, TEST_VAL);
            }
            assert_eq!(get(KEY).or(String::from(FAIL_VAL)), String::from(TEST_VAL));
            assert_eq!(
                get(KEY).or_else(|| String::from(FAIL_VAL)),
                String::from(TEST_VAL)
            );
            assert_eq!(get(KEY).to_option(), Some(String::from(TEST_VAL)));
            unsafe {
                env::remove_var(KEY);
            }
        }

        #[test]
        fn get_default_when_var_cannot_parse() {
            const KEY: &str = "get_default_when_var_cannot_parse";
            unsafe {
                env::set_var(KEY, TEST_VAL);
            }
            assert_eq!(get(KEY).or(1_i32), 1);
            assert_eq!(get(KEY).or_else(|| 1_i32), 1);
            assert_eq!(get(KEY).to_option::<i32>(), None);
            unsafe {
                env::remove_var(KEY);
            }
        }

        #[test]
        fn get_default_when_var_not_set() {
            const KEY: &str = "get_default_when_var_not_set";
            assert_eq!(get(KEY).or(String::from(FAIL_VAL)), String::from(FAIL_VAL));
            assert_eq!(
                get(KEY).or_else(|| String::from(FAIL_VAL)),
                String::from(FAIL_VAL)
            );
            assert_eq!(get(KEY).to_option::<String>(), None);
        }
    }
}
