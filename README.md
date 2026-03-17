# rust-env-var-lib

This is a library for Rust apps to make reading environment variables a little easier. It provides automatic parsing to a desired type, with fallbacks to a default value built into the main workflow.

### Example
```Rust
use env_var;

// `my_val` will be an i32 parsed from the value of the environment variable. 
// If the variable isn't set or the value cannot be coerced to an i32, the 
// default of 42 will be used.
let my_val = env_var::get("MY_KEY").or(42_i32);

// `my_str_val` will be a String, either parsed from the environment variable or generated
// by calling the lambda function passed to `or_else`. 
let my_str_val = env_var::get("MY_STR_VAL").or_else(|| String::from("some string"));

// `my_option_val` and `my_other_syntax_val` will both be instances of `Option<String>`.
// They will be `Some` if the environment variable was set, or `None` if it was not.
let my_option_val = env_var::get("MY_OPTION_VAL").to_option::<String>();
let my_other_syntax_val: Option<String> = env_var::get("MY_OPTION_VAL").to_option();

// Calling `expect` will auto-unpack and parse to the desired type. It will panic if the 
// value is not set or if the value cannot be parsed to the desired type.
let panicking_val = env_var::expect::<f64>("MY_FLOATING_POINT_VAL");
let other_panicking_val: String = env_var::expect("SOME_REQUIRED_ENV_VAL"); 
```