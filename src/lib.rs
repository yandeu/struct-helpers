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

pub type HelpersResult = Result<(), &'static str>;

pub trait Helpers {
    fn run_helpers(&mut self) -> Result<(), String>;
}

pub fn trim(s: &mut String) -> HelpersResult {
    *s = s.trim().to_string();
    Ok(())
}

pub fn to_lower_case(s: &mut String) -> HelpersResult {
    *s = s.to_lowercase();
    Ok(())
}

pub fn to_lower_case_optional(s_opt: &mut Option<String>) -> HelpersResult {
    if let Some(ref mut s) = s_opt {
        *s = s.to_lowercase();
    }
    Ok(())
}

pub fn to_upper_case(s: &mut String) -> HelpersResult {
    *s = s.to_uppercase();
    Ok(())
}

#[cfg(feature = "regex")]
pub fn regex(s: &mut str, re: &'static str) -> HelpersResult {
    let re = Regex::new(re).unwrap();
    match re.is_match(s) {
        true => Ok(()),
        false => Err("string does not match regex"),
    }
}

#[cfg(feature = "regex")]
pub fn regex_optional(s_opt: &mut Option<String>, re: &'static str) -> HelpersResult {
    if let Some(ref mut s) = s_opt {
        return regex(s, re);
    }
    Ok(())
}
