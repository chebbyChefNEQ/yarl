use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut tracker = HashSet::new();
        let mut idx1 = 0;
        let mut idx2 = 0;
        let bytes = s.as_bytes();
        let mut max = 0;
        while idx2 < s.len() && idx1 < s.len() {
            // if the new char in the window is present in the tracker
            // retract start fo the window till we remove the char
            // Other wise keep going
            if tracker.contains(&bytes[idx2]) {
                tracker.remove(&bytes[idx1]);
                idx1 += 1;
            } else {
                tracker.insert(&bytes[idx2]);
                idx2 += 1;
            }
            // need to insert the last char no matter what
            max = std::cmp::max(max, tracker.len());
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    fn do_test(s: &str, expected: i32) {
        assert_eq!(
            super::Solution::length_of_longest_substring(s.to_string()),
            expected
        );
    }

    #[test]
    fn simple() {
        do_test("abcabcbb", 3);
    }

    #[test]
    fn repeats() {
        do_test("aaaaaaabbbbbbb", 2);
    }
}
