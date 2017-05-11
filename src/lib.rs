extern crate regex;
extern crate rand;

use std::str;
use std::os::raw::c_int;
use regex::RegexBuilder;
use rand::{Rng, weak_rng};

// Counts how many of a specified number of random transformations of a string that matches a certain pattern, returns the number of matches or negative for error
#[no_mangle]
pub extern fn match_random_strings(num_strings: usize) -> c_int {
    let pattern = "^[a-zA-Z0-9_\\s\\r\\n\\t]*$";
    let matcher = match RegexBuilder::new(pattern).case_insensitive(true).dot_matches_new_line(true).multi_line(true).build() {
        Ok(regex) => regex,
        Err(_) => return -1
    };
    let mut value = r"<script>alert(123)</script><div><frame>Second Div</frame><p>1=1;\\x00</p></div></body></html>".as_bytes().to_owned();
    let mut matches = 0;
    let mut rng = weak_rng();
    for _ in 0..num_strings {
        rng.shuffle(&mut (value.clone()));    // Should be ok as long as the string only contains ascii
        matches += match str::from_utf8(&value) {
            Ok(value_str) => if matcher.is_match(value_str) { 1 } else { 0 },
            _ => return -2
        }
    }
    matches
}


#[cfg(test)]
mod tests {
    use match_random_strings;

    #[test]
    fn test_regex() {
        let result = match_random_strings(1000);
        println!("Result was {}", result);
        assert!(result >= 0);
    }
}
