# Struct Helpers

[![dependency status](https://deps.rs/repo/github/yandeu/struct-helpers/status.svg)](https://deps.rs/repo/github/yandeu/struct-helpers)
[![CI](https://github.com/yandeu/struct-helpers/actions/workflows/main.yml/badge.svg)](https://github.com/yandeu/struct-helpers/actions/workflows/main.yml)

## Install

```toml
[dependencies]
struct-helpers = { git = "https://github.com/yandeu/struct-helpers" }
```

## Example

```rust
use struct_helpers::{to_lower_case, to_upper_case, trim, Helpers, HelpersResult};

#[derive(Helpers)]
pub struct User {
    #[helper(trim, to_upper_case)]
    name: String,
    #[helper(times_two)]
    age: i32,
    #[helper(with_param(10, "times!"))]
    thanks: String
}

fn times_two(n: &mut i32) -> HelpersResult {
    *n *= 2;
    Ok(())
}

fn with_param(s: &mut String, x: i32, y: &str) -> HelpersResult {
    *s = format!("{} {} {}", s, x * 10, y);
    Ok(())
}

fn main() {
    let mut u = User {
        name: " John ".to_string(),
        age: 20,
        thanks: "Thank you".to_string()
    };

    u.run_helpers().unwrap();

    assert_eq!(u.name, "JOHN");
    assert_eq!(u.age, 40);
    assert_eq!(u.thanks, "Thank you 100 times!");
}
```

## Validate with Regex

```rust
// features = [ "regex" ]
use struct_helpers::{regex, regex_optional, Helpers};

#[derive(Debug, Default, Helpers)]
struct User {
    #[helper(regex(r"^[a-f0-9]{6}$"))]
    id_1: String,
    #[helper(regex(r"^[a-f0-9]{6}$"))]
    id_2: Option<String>,
}

fn main() {
    let mut user = User {
        id_1: String::from("123abc"),
        id_2: Some(String::from("123abc")),
    };

    user.run_helpers().unwrap();
}
```

## Rocket.rs Guard

```toml
[dependencies]
struct-helpers = { git = "https://github.com/yandeu/struct-helpers", features = [ "rocket" ] }
```

```rust
#[macro_use] extern crate rocket;
use rocket::serde::{ json::Json, Deserialize };
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

# Development

Run  
`cargo watch -x 'run --all-features'`

Test  
`cargo test --all --all-features -- --nocapture`
