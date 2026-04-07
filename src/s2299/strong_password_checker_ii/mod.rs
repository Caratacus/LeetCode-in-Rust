// Problem 2299: strong password checker ii

pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let chars: Vec<char> = password.chars().collect();
        let n = chars.len();
        if n < 8 {
            return false;
        }
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut has_special = false;
        let special: std::collections::HashSet<char> =
            "!@#$%^&*()-+".chars().collect();
        for i in 0..n {
            let c = chars[i];
            if i > 0 && chars[i] == chars[i - 1] {
                return false;
            }
            if c.is_ascii_lowercase() {
                has_lower = true;
            } else if c.is_ascii_uppercase() {
                has_upper = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            } else if special.contains(&c) {
                has_special = true;
            }
        }
        has_lower && has_upper && has_digit && has_special
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void strongPasswordCheckerII()
    //   assertThat(new Solution().strongPasswordCheckerII("IloveLe3tcode!"), equalTo(true));
    #[test]
    fn test_strong_password_checker_ii() {
        // TODO: 翻译 Java 测试
    }

    // Java: void strongPasswordCheckerII2()
    //   assertThat(new Solution().strongPasswordCheckerII("Me+You--IsMyDream"), equalTo(false));
    #[test]
    fn test_strong_password_checker_ii2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void strongPasswordCheckerII3()
    //   assertThat(new Solution().strongPasswordCheckerII("1aB!"), equalTo(false));
    #[test]
    fn test_strong_password_checker_ii3() {
        // TODO: 翻译 Java 测试
    }

    // Java: void strongPasswordCheckerII4()
    //   assertThat(
    //   new Solution()
    //   .strongPasswordCheckerII(
    //   "ecuwcfoyajkolntovfniplayrxhzpmhrkhzonopcwxgupzhoupw"),
    //   equalTo(false));
    #[test]
    fn test_strong_password_checker_ii4() {
        // TODO: 翻译 Java 测试
    }

    // Java: void strongPasswordCheckerII5()
    //   assertThat(new Solution().strongPasswordCheckerII("\"|{}\"|{}"), equalTo(false));
    #[test]
    fn test_strong_password_checker_ii5() {
        // TODO: 翻译 Java 测试
    }
}
