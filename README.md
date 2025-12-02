# rust-env-var-lib

This is a library for Rust apps to make reading environment variables a little easier. It provides automatic parsing to a desired type, with fallbacks to a default value built into the main workflow.

### Example
```Rust
{
    use env_var;

    // my_val will be a String parsed from the value of the environment variable, 
    // or 42 if the variable isn't set or the value cannot be coerced to an i32
    let my_val = env_var::get("MY_KEY").or(42_i32);
}
```