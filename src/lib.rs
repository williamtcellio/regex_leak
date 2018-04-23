extern crate regex;
extern crate rand;

use std::sync::Arc;
use regex::{RegexBuilder, Regex};

// Counts how many of a specified number of random transformations of a string that matches a certain pattern, returns the number of matches or negative for error
#[no_mangle]
pub extern fn match_strings(matcher: Arc<Regex>, num_threads: usize) {
    let value = r"something something something something";
    for i in 0..num_threads {
        let value_str = value.clone().as_ref();
        let matcher_clone = matcher.clone();
        let print = i % 10000 == 0;
        ::std::thread::spawn(move || {
            if print {
                println!("{}", i);
            }
            matcher_clone.is_match(value_str);
        }).join().unwrap();
    }
}


#[test]
fn test_regex() {
    let pattern = "^[a-zA-Z0-9_\\s\\r\\n\\t]*$";
    let matcher = match RegexBuilder::new(pattern).case_insensitive(true).dot_matches_new_line(true).multi_line(true).build() {
        Ok(regex) => regex,
        Err(err) => panic!("Failed to initialize {}", err)
    };
    match_strings(Arc::new(matcher), 100000000);
}
