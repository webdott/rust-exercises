use std::cmp::max;

// Run this file with `cargo test --test 03_interleave`.

// TODO: Implement a function called `interleave`, which will take two string slices and return
// a string that contains the characters from the two input strings interleaved.
// The first character of the output should start with the first character of the first argument.
// See tests for details.
//
// Can you write the function without any explicit indexing (`str[index]`)?
//
// Hint: you can use `string.chars()` to create an iterator over the Unicode characters of a string.

fn interleave(a: &str, b: &str) -> String {
    let max_len = max(a.len(), b.len());

    let mut res = String::new();

    for i in 0..max_len {
        let (a_str, b_str) = (a.chars().nth(i), b.chars().nth(i));

        match a_str {
            Some(str) => {
                res += &str.to_string()
            },
            None => {}
        }

        match b_str {
            Some(str) => {
                res += &str.to_string()
            },
            None => {}
        }
    }

    res
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::interleave;

    #[test]
    fn interleave_empty() {
        assert_eq!(interleave("", ""), "");
    }

    #[test]
    fn interleave_only_left() {
        assert_eq!(interleave("a", ""), "a");
        assert_eq!(interleave("zxjas", ""), "zxjas");
    }

    #[test]
    fn interleave_only_right() {
        assert_eq!(interleave("", "z"), "z");
        assert_eq!(interleave("", "foobar"), "foobar");
    }

    #[test]
    fn interleave_same_length() {
        assert_eq!(interleave("abcdef", "012345"), "a0b1c2d3e4f5");
    }

    #[test]
    fn interleave_first_longer() {
        assert_eq!(
            interleave("Programming Rust", "O'Reilly"),
            "POr'oRgerialmlmying Rust"
        );
    }

    #[test]
    fn interleave_second_longer() {
        assert_eq!(
            interleave("ahoj, jak se máš?", "díky za optání, mám se dobře"),
            "adhíokjy,  zjaa ko psteá nmíá,š ?mám se dobře"
        );
    }
}