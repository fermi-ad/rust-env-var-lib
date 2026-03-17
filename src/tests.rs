//! Tests Module

use super::env_var;

// This key/value pair gets set in the .cargo/config.toml file and is picked up during the `cargo test` command.
const NUMERIC_KEY: &str = "NUMERIC_KEY";
const NUMERIC_VALUE: u32 = 1;

// This key/value pair is not set anywhere, so is used in the tests as an "unset" value.
const OTHER_STRING_KEY: &str = "OTHER_STRING_KEY";
const OTHER_STRING_VALUE: &str = "OTHER_STRING_VALUE";

// This key/value pair gets set in the .cargo/config.toml file and is picked up during the `cargo test` command.
const STRING_KEY: &str = "STRING_KEY";
const STRING_VALUE: &str = "STRING_VALUE";

#[test]
fn get_var() {
    assert_eq!(
        env_var::get(STRING_KEY).or(String::from(OTHER_STRING_VALUE)),
        String::from(STRING_VALUE)
    );
    assert_eq!(
        env_var::get(STRING_KEY).or_else(|| String::from(OTHER_STRING_VALUE)),
        String::from(STRING_VALUE)
    );
    assert_eq!(
        env_var::get(STRING_KEY).to_option(),
        Some(String::from(STRING_VALUE))
    );
}

#[test]
fn get_default_when_var_cannot_parse() {
    assert_eq!(env_var::get(STRING_KEY).or(NUMERIC_VALUE), NUMERIC_VALUE);
    assert_eq!(
        env_var::get(STRING_KEY).or_else(|| NUMERIC_VALUE),
        NUMERIC_VALUE
    );
    assert_eq!(env_var::get(STRING_KEY).to_option::<u32>(), None);
}

#[test]
fn get_default_when_var_not_set() {
    assert_eq!(
        env_var::get(OTHER_STRING_KEY).or(String::from(STRING_VALUE)),
        String::from(STRING_VALUE)
    );
    assert_eq!(
        env_var::get(OTHER_STRING_KEY).or_else(|| String::from(STRING_VALUE)),
        String::from(STRING_VALUE)
    );
    assert_eq!(env_var::get(OTHER_STRING_KEY).to_option::<String>(), None);
}

#[test]
fn expect_var_returns_when_present() {
    let val: u32 = env_var::expect(NUMERIC_KEY);
    assert_eq!(val, NUMERIC_VALUE);
}

#[test]
#[should_panic]
fn expect_var_panics_when_missing() {
    let _ = env_var::expect::<String>(OTHER_STRING_KEY);
}

#[test]
#[should_panic]
fn expect_var_panics_when_not_parseable() {
    let _ = env_var::expect::<u32>(STRING_KEY);
}
