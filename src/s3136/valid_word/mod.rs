// Problem 3136: valid word
// #Easy #String #2024_07_15_Time_1 ms(99.12%)  Space 42.10 MB (62.25%)

pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut has_vowel = false;
        let mut has_consonant = false;

        for c in word.chars() {
            if c.is_ascii_alphabetic() {
                let ch = c.to_ascii_lowercase();
                if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                    has_vowel = true;
                } else {
                    has_consonant = true;
                }
            } else if !c.is_ascii_digit() {
                return false;
            }
        }

        has_vowel && has_consonant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid(String::from("234Adas")), true);
    }

    #[test]
    fn test_is_valid2() {
        assert_eq!(Solution::is_valid(String::from("b3")), false);
    }

    #[test]
    fn test_is_valid3() {
        assert_eq!(Solution::is_valid(String::from("a3$e")), false);
    }

    #[test]
    fn test_is_valid4() {
        assert_eq!(Solution::is_valid(String::from("a")), false);
        assert_eq!(Solution::is_valid(String::from("ab")), false);
        assert_eq!(Solution::is_valid(String::from("1")), false);
        assert_eq!(Solution::is_valid(String::from("1a")), false);
        assert_eq!(Solution::is_valid(String::from("")), false);
    }

    #[test]
    fn test_is_valid5() {
        assert_eq!(Solution::is_valid(String::from("aei")), false);
        assert_eq!(Solution::is_valid(String::from("AEI")), false);
        assert_eq!(Solution::is_valid(String::from("Aei")), false);
        assert_eq!(Solution::is_valid(String::from("uuu")), false);
    }

    #[test]
    fn test_is_valid6() {
        assert_eq!(Solution::is_valid(String::from("bcdfg")), false);
        assert_eq!(Solution::is_valid(String::from("BCD")), false);
        assert_eq!(Solution::is_valid(String::from("xyz")), false);
        assert_eq!(Solution::is_valid(String::from("QWRTY")), false);
    }

    #[test]
    fn test_is_valid7() {
        assert_eq!(Solution::is_valid(String::from("abc")), true);
        assert_eq!(Solution::is_valid(String::from("bac")), true);
        assert_eq!(Solution::is_valid(String::from("AeIbcD")), true);
        assert_eq!(Solution::is_valid(String::from("tree")), true);
        assert_eq!(Solution::is_valid(String::from("skyE")), true);
    }

    #[test]
    fn test_is_valid8() {
        assert_eq!(Solution::is_valid(String::from("a1b2c")), true);
        assert_eq!(Solution::is_valid(String::from("1a2b")), true);
        assert_eq!(Solution::is_valid(String::from("b2c4e")), true);
        assert_eq!(Solution::is_valid(String::from("123")), false);
    }

    #[test]
    fn test_is_valid10() {
        assert_eq!(Solution::is_valid(String::from("a#b")), false);
        assert_eq!(Solution::is_valid(String::from("@ab")), false);
        assert_eq!(Solution::is_valid(String::from("ab!")), false);
        assert_eq!(Solution::is_valid(String::from("c_d")), false);
        assert_eq!(Solution::is_valid(String::from("a.b")), false);
        assert_eq!(Solution::is_valid(String::from("a@b")), false);
    }

    #[test]
    fn test_is_valid11() {
        assert_eq!(Solution::is_valid(String::from("AbC")), true);
        assert_eq!(Solution::is_valid(String::from("BacE1")), true);
        assert_eq!(Solution::is_valid(String::from("zEi")), true);
    }

    #[test]
    fn test_is_valid12() {
        assert_eq!(Solution::is_valid(String::from("a1b")), true);
        assert_eq!(Solution::is_valid(String::from("ab1")), true);
        assert_eq!(Solution::is_valid(String::from("1ab")), true);
        assert_eq!(Solution::is_valid(String::from("1a")), false);
        assert_eq!(Solution::is_valid(String::from("1b")), false);
    }
}
