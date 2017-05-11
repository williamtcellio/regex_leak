extern crate regex;
extern crate rand;

pub mod matcher {
    use std::str;
    use std::os::raw::c_int;
    use regex::RegexBuilder;
    use rand::{Rng, weak_rng};

    // Counts how many of a specified number of random transformations of a string that matches a certain pattern, returns the number of matches or negative for error
    pub extern fn match_random_strings(num_strings: usize) -> c_int {
        let pattern = r"(?:<(script|iframe|embed|frame|frameset|object|img|applet|body|html|style|layer|link|ilayer|meta|bgsound))";
        let matcher = match RegexBuilder::new(pattern).case_insensitive(true).dot_matches_new_line(true).multi_line(true).build() {
            Ok(regex) => regex,
            Err(_) => return -1
        };
        let mut value = r"<script>alert(123)</script><div><frame>Second Div</frame></div></body></html>".as_bytes().to_owned();
        let mut matches = 0;
        let mut rng = weak_rng();
        for _ in 0..num_strings {
            rng.shuffle(&mut value);    // Should be ok as long as the string only contains ascii
            matches += match str::from_utf8(&value) {
                Ok(value_str) => if matcher.is_match(value_str) { 1 } else { 0 },
                _ => return -2
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use match_random_strings;

    #[test]
    fn test_regex() {
        assert!(match_random_strings(1000) >= 0);
    }
}
