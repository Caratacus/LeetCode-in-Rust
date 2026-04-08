// Problem 3110: Score of a String
// #Easy #String #2024_04_27_Time_1_ms_(99.93%)_Space_41.4_MB_(99.03%)

pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut sum = 0;
        for i in 0..bytes.len() - 1 {
            sum += (bytes[i] as i32 - bytes[i + 1] as i32).abs();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void scoreOfString()
    //   assertThat(new Solution().scoreOfString("hello"), equalTo(13));
    #[test]
    fn test_score_of_string() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    }

    // Java: void scoreOfString2()
    //   assertThat(new Solution().scoreOfString("zaz"), equalTo(50));
    #[test]
    fn test_score_of_string2() {
        assert_eq!(Solution::score_of_string("zaz".to_string()), 50);
    }
}
