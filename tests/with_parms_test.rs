use struct_helpers::{Helpers, HelpersResult};

#[test]
fn it_works_with_params() {
    #[derive(Debug, Default, Helpers)]
    struct Point {
        #[helper(validate_len, with_param(10, "hello"))]
        x: String,
    }

    fn with_param(x: &mut String, y: i32, z: &str) -> HelpersResult {
        x.push_str(y.to_string().as_str());
        x.push_str(z);

        Ok(())
    }

    fn validate_len(x: &String) -> HelpersResult {
        println!("len {} {}", x.len(), x.len() > 5);
        match x.len() <= 5 {
            true => Ok(()),
            false => Err("validate_len failed"),
        }
    }

    let mut p = Point {
        x: "hello".to_string(),
    };

    p.run_helpers().unwrap();

    assert_eq!(p.x, "hello10hello");
}
