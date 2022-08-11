// test code in README file
#[doc = include_str!("../README.md")]
pub use struct_helpers_macro::Helpers;

pub fn trim(s: &mut String) -> bool {
    *s = s.trim().to_string();
    true
}

pub fn to_lower_case(s: &mut String) -> bool {
    *s = s.to_lowercase();
    true
}

pub fn to_upper_case(s: &mut String) -> bool {
    *s = s.to_uppercase();
    true
}
