// Problem 3168: minimum number of chairs in a waiting room
// #Easy #String #Simulation #2024_06_06_Time_1_ms_(100.00%)_Space_41.9_MB_(67.53%)

pub struct Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut count = 0;
        let mut ans = i32::MIN;
        for ch in s.chars() {
            if ch == 'E' {
                count += 1;
                ans = ans.max(count);
            } else {
                count -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_chairs() {
        assert_eq!(Solution::minimum_chairs("EEEEEEE".to_string()), 7);
    }

    #[test]
    fn test_minimum_chairs2() {
        assert_eq!(Solution::minimum_chairs("ELELEEL".to_string()), 2);
    }

    #[test]
    fn test_minimum_chairs3() {
        assert_eq!(Solution::minimum_chairs("ELEELEELLL".to_string()), 3);
    }
}
