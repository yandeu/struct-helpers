use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use struct_helpers::{to_lower_case, trim, Helpers};

// #[derive(Debug, Default, Helpers)]
// struct SubPoint {
//     #[helper(times_two)]
//     a: i32,
// }

#[skip_serializing_none]
#[derive(Debug, Default, Helpers, Deserialize, Serialize)]
struct Point {
    #[serde(rename = "_id")]
    id: Option<String>,
    #[helper(validate_len, with_param(3, "hello"))]
    x: String,
    // #[helper(hello)]
    // y: Option<String>,
    // #[helper(times_two)]
    // z: i32,
    // #[helper(mod_sub_point)]
    // sub: SubPoint,
    #[helper(trim, to_lower_case)]
    t: String,
}

// fn with_param(x: &String, y: &str, z: &str) -> bool {
//     println!("{}/{}/{}", x, y, z);
//     return true;
// }

fn with_param(x: &String, y: i32, z: &str) -> bool {
    println!("{}/{}/{}", x, y, z);
    true
}

fn validate_len(x: &String) -> bool {
    println!("len {} {}", x.len(), x.len() > 5);
    x.len() > 5
}

// fn mod_sub_point(p: &mut SubPoint) -> bool {
//     p.a += 10;
//     true
// }

// generate the "optional" wrapper automatically
// fn hello_optional(s_opt: &mut Option<String>) -> bool {
//     if let Some(ref mut s) = s_opt {
//         return hello(s);
//     }
//     false
// }

// fn hello(s: &mut String) -> bool {
//     s.push_str(", hello");
//     true
// }

// fn trim(s: &mut String) -> bool {
//     *s = s.trim().to_string();
//     true
// }

// fn times_two(n: &mut i32) -> bool {
//     *n *= 2;
//     false
// }

// impl Point {
//     fn valid(&mut self) {
//         hello(&mut self.x);
//     }
// }

// fn func_string(field: &str) -> String {
//     let mut sanitizer = StringSanitizer::from(field);
//     sanitizer.trim();
//     sanitizer.get()
// }

fn main() {
    // let sub = SubPoint { a: 3 };
    let mut p = Point {
        x: " 5".to_string(),
        // y: " 8 ".to_string(),
        // z: 2,
        // sub: sub,
        t: " triMM me  ".to_string(),
        ..Default::default()
    };

    // p.y = Some("  DDD".to_string());

    let success = p.run_helpers();
    if !success {
        println!("Something went wrong..")
    }

    println!("x = {}", &p.x);
    println!("t = {}", &p.t);

    // p.sub.run_helpers();
    // let s = p.point_summation();

    // hello_optional(&mut p.y);s

    // println!("= {} {} ", &p.x, &p.y.unwrap_or_default());
    // println!("sub = {}", &p.sub.a);
}
