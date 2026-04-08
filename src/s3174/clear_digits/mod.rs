// Problem 3174: clear digits
// #Easy #String #Hash_Table #Simulation #2024_06_12_Time_1_ms_(100.00%)_Space_42.1_MB_(96.47%)

pub struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result = String::new();
        for ch in s.chars() {
            if ch >= '0' && ch <= '9' {
                if !result.is_empty() {
                    result.pop();
                }
            } else {
                result.push(ch);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void clearDigits()
    //   assertThat(new Solution().clearDigits("abc"), equalTo("abc"));
    #[test]
    fn test_clear_digits() {
        assert_eq!(Solution::clear_digits("abc".to_string()), "abc");
    }

    // Java: void clearDigits2()
    //   assertThat(new Solution().clearDigits("cb34"), equalTo(""));
    #[test]
    fn test_clear_digits2() {
        assert_eq!(Solution::clear_digits("cb34".to_string()), "");
    }
}
