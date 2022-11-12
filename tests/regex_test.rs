use struct_helpers::{regex, regex_optional, Helpers};

#[test]
fn regex_validation() {
    #[derive(Debug, Default, Helpers)]
    struct User {
        #[helper(regex(r"^[a-f0-9]{6}$"))]
        id_1: String,
        #[helper(regex(r"^[a-f0-9]{6}$"))]
        id_2: Option<String>,
    }

    let mut user = User {
        id_1: String::from("123abc"),
        id_2: Some(String::from("123abc")),
    };

    user.run_helpers().unwrap();
}
