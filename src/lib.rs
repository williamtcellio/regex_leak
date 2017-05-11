extern crate regex;

use std::slice;
use std::str;
use std::os::raw::c_int;
use regex::RegexBuilder;

// Matches the value string to the regex pattern, returns 1 for match, 0 for non-match, negative for error
pub extern fn match_regex(pattern: *const u8, pattern_len: usize, value: *const u8, value_len: usize) -> c_int {
    let pattern_str = match str::from_utf8(unsafe{slice::from_raw_parts(pattern, pattern_len)}) {
        Ok(s) => s,
        _ => return -1
    };
    let value_str = match str::from_utf8(unsafe { slice::from_raw_parts(value, value_len) }) {
        Ok(s) => s,
        _ => return -2
    };
    match RegexBuilder::new(pattern_str).case_insensitive(true).dot_matches_new_line(true).multi_line(true).build() {
        Ok(regex) => if regex.is_match(value_str) { 1 } else { 0 },
        Err(_) => -3
    }
}

#[cfg(test)]
mod tests {
    use match_regex;

    #[test]
    fn test_regex() {
        let pattern = r"(?:<(script|iframe|embed|frame|frameset|object|img|applet|body|html|style|layer|link|ilayer|meta|bgsound))";
        let value = r"<script>alert(123)</script>";
        assert_eq!(match_regex(pattern.as_ptr(), pattern.len(), value.as_ptr(), value.len()), 1);
    }
}
