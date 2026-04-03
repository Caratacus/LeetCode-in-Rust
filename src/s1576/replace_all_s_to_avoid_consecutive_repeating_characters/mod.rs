// Problem 1576: Replace All ?'s to Avoid Consecutive Repeating Characters
// #Easy #String
// #Big_O_Time_O(n)_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let s_bytes = s.as_bytes();
        let len = s.len();
        let mut result = Vec::with_capacity(len);

        for i in 0..len {
            let c = s_bytes[i];
            if c == b'?' {
                let mut replace_char = b'a';
                let left_char = if i == 0 { b'?' } else { result[i - 1] };
                let right_char = s_bytes[std::cmp::min(i + 1, len - 1)];
                while replace_char == left_char || replace_char == right_char {
                    replace_char += 1;
                }
                result.push(replace_char);
            } else {
                result.push(c);
            }
        }

        String::from_utf8(result).unwrap()
    }
}
