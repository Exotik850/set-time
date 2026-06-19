# set-time-rs

Small cross-platform utility crate to set the current time of the User's machine

## Example

Add to your project's dependencies:

`cargo add set-time`

and run the `set_time` function:

```rust
use set_time::{set_time, DateTime};

fn main() {
  // Set the system time to January 1, 2020
  let new_time_str = "2020-01-01 00:00:00";
  let new_time = DateTime::parse_from_str(new_time_str, "%Y-%m-%d %H:%M:%S")
      .expect("Failed to parse time");
  set_time(new_time).expect("Failed to set system time");
}
```

## License

This project is covered under the [MIT License](LICENSE.md)