pub mod env_var {
    use std::{
        env::{self, VarError},
        fmt::Display,
        str::FromStr,
    };
    use tracing::{error, warn};
    pub struct EnvVal {
        var_name: String,
        result: Result<String, VarError>,
    }
    impl EnvVal {
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
    }

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
        fn do_tests() {
            let get_var_result = get_var();
            let parse_failure_result = get_default_when_var_cannot_parse();
            let var_not_set_result = get_default_when_var_not_set();

            let result = format!(
                "{}{}{}",
                get_var_result.unwrap_or_default(),
                parse_failure_result.unwrap_or_default(),
                var_not_set_result.unwrap_or_default()
            );
            if result != String::new() {
                panic!("\nFAILURE: {}\n", result);
            }
        }

        fn get_var() -> Option<String> {
            unsafe {
                env::set_var(TEST_VAL, TEST_VAL);
            }
            let response = match get(TEST_VAL).or(String::from(FAIL_VAL)) == String::from(TEST_VAL)
            {
                true => None,
                false => Some(String::from("get_var()    ")),
            };
            unsafe {
                env::remove_var(TEST_VAL);
            }
            response
        }

        fn get_default_when_var_cannot_parse() -> Option<String> {
            unsafe {
                env::set_var(TEST_VAL, TEST_VAL);
            }
            let response = match get(TEST_VAL).or(1_i32) {
                1 => None,
                _ => Some(String::from("get_default_when_var_cannot_parse()    ")),
            };
            unsafe {
                env::remove_var(TEST_VAL);
            }
            response
        }

        fn get_default_when_var_not_set() -> Option<String> {
            if get(TEST_VAL).or(String::from(FAIL_VAL)) == String::from(FAIL_VAL) {
                None
            } else {
                Some(String::from("get_default_when_var_not_set()"))
            }
        }
    }
}
