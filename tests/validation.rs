use struct_helpers::Helpers;

#[test]
fn validation() {
    #[derive(Debug, Default, Helpers)]
    struct User {
        #[helper(validate_len(6))]
        id: String,
    }

    fn validate_len(x: &String, len: usize) -> bool {
        x.len() == len
    }

    let mut user1 = User {
        id: String::from("123ab"),
    };

    let mut user2 = User {
        id: String::from("123abc"),
    };

    let Err(e) = user1.run_helpers() else {
      panic!()
    };

    let Ok(_) = user2.run_helpers() else {
      panic!()
    };

    assert_eq!(e, "Error in fn validate_len for field id");
}
