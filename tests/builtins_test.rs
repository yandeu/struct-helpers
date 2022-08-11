use struct_helpers::{to_lower_case, to_upper_case, trim, Helpers};

#[test]
fn build_in_functions() {
    #[derive(Debug, Helpers)]
    struct Point {
        #[helper(trim, to_upper_case)]
        x: String,
        #[helper(trim, to_lower_case)]
        y: String,
    }

    let mut p = Point {
        x: " Hello ".to_string(),
        y: " Hello ".to_string(),
    };

    let success = p.run_helpers();

    assert_eq!(success, true);
    assert_eq!(p.x, "HELLO");
    assert_eq!(p.y, "hello");
}
