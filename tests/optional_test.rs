use struct_helpers::{Helpers, HelpersResult};

#[test]
fn it_works_with_option() {
    #[derive(Debug, Helpers)]
    struct Point {
        #[helper(hello)]
        x: String,
        #[helper(hello)]
        y: Option<String>,
    }

    fn hello(s: &mut String) -> HelpersResult {
        let mut tmp = "hello, ".to_string();
        tmp.push_str(s);
        *s = tmp;
        Ok(())
    }

    // Option wrapper for hello()
    fn hello_optional(s_opt: &mut Option<String>) -> HelpersResult {
        if let Some(ref mut s) = s_opt {
            return hello(s);
        }
        Ok(())
    }

    let mut p = Point {
        x: "x".to_string(),
        y: Some("y".to_string()),
    };

    p.run_helpers().unwrap();

    assert_eq!(p.x, "hello, x");
    assert_eq!(p.y.unwrap(), "hello, y");
}
