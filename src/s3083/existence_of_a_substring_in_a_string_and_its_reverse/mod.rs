// Problem 3083: existence of a substring in a string and its reverse
// #Easy #String #Hash_Table

pub struct Solution;

impl Solution {
    pub fn is_substring_present(s: String) -> bool {
        if s.len() == 1 {
            return false;
        }
        let rev: String = s.chars().rev().collect();
        for i in 0..s.len() - 1 {
            let sub = &s[i..i + 2];
            if rev.contains(sub) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void isSubstringPresent()
    //   assertThat(new Solution().isSubstringPresent("leetcode"), equalTo(true));
    #[test]
    fn test_is_substring_present() {
        assert_eq!(Solution::is_substring_present("leetcode".to_string()), true);
    }

    // Java: void isSubstringPresent2()
    //   assertThat(new Solution().isSubstringPresent("abcba"), equalTo(true));
    #[test]
    fn test_is_substring_present2() {
        assert_eq!(Solution::is_substring_present("abcba".to_string()), true);
    }

    // Java: void isSubstringPresent3()
    //   assertThat(new Solution().isSubstringPresent("abcd"), equalTo(false));
    #[test]
    fn test_is_substring_present3() {
        assert_eq!(Solution::is_substring_present("abcd".to_string()), false);
    }
}
