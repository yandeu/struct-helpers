# Struct Helpers

[![CI](https://github.com/yandeu/struct-helpers/actions/workflows/main.yml/badge.svg)](https://github.com/yandeu/struct-helpers/actions/workflows/main.yml)

## Install

```toml
[dependencies]
struct_helpers = { git = "https://github.com/yandeu/struct-helpers" }
```

## Example

```rust
use struct_helpers::{to_lower_case, to_upper_case, trim, Helpers};

#[derive(Helpers)]
pub struct User {
    #[helper(trim, to_upper_case)]
    name: String,
    #[helper(times_two)]
    age: i32,
    #[helper(with_param(10, "times!"))]
    thanks: String
}

fn times_two(n: &mut i32) -> bool {
    *n *= 2;
    true
}

fn with_param(s: &mut String, x: i32, y: &str) -> bool {
    *s = format!("{} {} {}", s, x * 10, y);
    true
}

fn main() {
    let mut u = User {
        name: " John ".to_string(),
        age: 20,
        thanks: "Thank you".to_string()
    };

    let success = u.run_helpers();

    assert_eq!(success, true);
    assert_eq!(u.name, "JOHN");
    assert_eq!(u.age, 40);
    assert_eq!(u.thanks, "Thank you 100 times!");
}
```

## Rocket.rs Guard

```toml
[dependencies]
struct_helpers = { git = "https://github.com/yandeu/struct-helpers", features = [ "rocket" ] }
```

```rust
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use struct_helpers::{ to_lower_case, Helpers, rocket::guard::HelpersGuard };

#[derive(Debug, Deserialize, Helpers)]
struct User {
    #[helper(to_lower_case)]
    name: String,
}

#[post("/", format = "application/json", data = "<user>")]
fn hello(user: HelpersGuard<Json<User>>) -> String {
    user.into_deep_inner().name
}
```
