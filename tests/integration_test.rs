use struct_helpers::{Helpers, HelpersResult};

#[test]
fn it_works() {
    #[derive(Debug, Helpers)]
    struct Point {
        #[helper(trim, hello)]
        x: String,
        #[helper(trim, hello)]
        y: String,
        #[helper(times_two)]
        z: i32,
    }

    fn hello(s: &mut String) -> HelpersResult {
        s.push_str(", hello");
        Ok(())
    }

    fn trim(s: &mut String) -> HelpersResult {
        *s = s.trim().to_string();
        Ok(())
    }

    fn times_two(n: &mut i32) -> HelpersResult {
        *n *= 2;
        Ok(())
    }

    let mut p = Point {
        x: " 5".to_string(),
        y: " 8 ".to_string(),
        z: 2,
    };

    p.run_helpers().unwrap();

    assert_eq!(p.x, "5, hello");
    assert_eq!(p.y, "8, hello");
    assert_eq!(p.z, 4);
}
