// Problem 2283: check if number has equal digit count and digit value

pub struct Solution;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let digits: Vec<u32> = num.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let n = digits.len();
        for i in 0..n {
            let expected = digits[i] as usize;
            let count = digits.iter().filter(|&&d| d as usize == i).count();
            if expected != count {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void digitCount()
    //   assertThat(new Solution().digitCount("1210"), equalTo(true));
    #[test]
    fn test_digit_count() {
        // TODO: 翻译 Java 测试
    }

    // Java: void digitCount2()
    //   assertThat(new Solution().digitCount("030"), equalTo(false));
    #[test]
    fn test_digit_count2() {
        // TODO: 翻译 Java 测试
    }
}
