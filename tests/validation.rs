use struct_helpers::{Helpers, HelpersResult};

#[test]
fn validation() {
    #[derive(Debug, Default, Helpers)]
    struct User {
        #[helper(validate_len(6))]
        id: String,
    }

    fn validate_len(x: &String, len: usize) -> HelpersResult {
        match x.len() == len {
            true => Ok(()),
            false => Err("validate_len failed"),
        }
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

    assert_eq!(e, "validate_len failed");
}
