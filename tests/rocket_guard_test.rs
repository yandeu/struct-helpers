#![allow(dead_code)]

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::Deserialize;
use struct_helpers::{rocket::guard::HelpersGuard, Helpers};

pub fn to_lower_case(s: &mut String) -> bool {
    *s = s.to_lowercase();
    true
}

pub fn min_length(s: &mut String, n: i32) -> bool {
    s.len() >= n as usize
}

#[derive(Debug, Default, Deserialize, Helpers)]
struct User {
    #[helper(to_lower_case, min_length(5))]
    name: String,
    email: String,
    password: String,
}

#[post("/", format = "application/json", data = "<user>")]
fn hello(user: HelpersGuard<Json<User>>) -> String {
    let name = user.into_deep_inner().name.clone();
    name
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::hello))
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "John Doe",
                    "email": "j.doe@m.com",
                    "password": "123456"
                }"##,
            )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "john doe");
    }

    #[test]
    fn hello_world_wrong_name_length() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::hello))
            .header(ContentType::JSON)
            .body(
                r##"{
                    "name": "John",
                    "email": "j.doe@m.com",
                    "password": "123456"
                }"##,
            )
            .dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }
}
