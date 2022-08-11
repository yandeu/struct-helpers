use struct_helpers::Helpers;

#[test]
fn it_works_with_params() {
    #[derive(Debug, Default, Helpers)]
    struct Point {
        #[helper(validate_len, with_param(10, "hello"))]
        x: String,
    }

    fn with_param(x: &mut String, y: i32, z: &str) -> bool {
        x.push_str(y.to_string().as_str());
        x.push_str(z);

        return true;
    }

    fn validate_len(x: &String) -> bool {
        println!("len {} {}", x.len(), x.len() > 5);
        x.len() <= 5
    }

    let mut p = Point {
        x: "hello".to_string(),
    };

    let success = p.run_helpers();

    assert_eq!(success, true);
    assert_eq!(p.x, "hello10hello");
}
