// test code in README file
#[doc = include_str!("../README.md")]
// regex feature
#[cfg(feature = "regex")]
use regex::Regex;
// rocket feature
#[cfg(feature = "rocket")]
pub mod rocket;
// macro
pub use struct_helpers_macro::Helpers;

pub trait Helpers {
    fn run_helpers(&mut self) -> Result<(), String>;
}

pub fn trim(s: &mut String) -> bool {
    *s = s.trim().to_string();
    true
}

pub fn to_lower_case(s: &mut String) -> bool {
    *s = s.to_lowercase();
    true
}

pub fn to_lower_case_optional(s_opt: &mut Option<String>) -> bool {
    if let Some(ref mut s) = s_opt {
        *s = s.to_lowercase();
    }
    true
}

pub fn to_upper_case(s: &mut String) -> bool {
    *s = s.to_uppercase();
    true
}

#[cfg(feature = "regex")]
pub fn regex(s: &mut String, re: &'static str) -> bool {
    let re = Regex::new(re).unwrap();
    re.is_match(s)
}

#[cfg(feature = "regex")]
pub fn regex_optional(s_opt: &mut Option<String>, re: &'static str) -> bool {
    if let Some(ref mut s) = s_opt {
        let re = Regex::new(re).unwrap();
        return re.is_match(s);
    }
    true
}
