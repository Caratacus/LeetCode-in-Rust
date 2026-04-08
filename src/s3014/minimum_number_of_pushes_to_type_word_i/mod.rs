// Problem 3014: minimum number of pushes to type word i
// #Easy #String #Math #Greedy

pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let len = word.len();
        if len <= 8 {
            return len as i32;
        } else {
            let mut iteration = 1;
            let mut len = len;
            let mut count = 0;
            while len > 0 {
                if len >= 8 {
                    count += 8 * iteration;
                    len -= 8;
                } else {
                    count += len * iteration;
                    len = 0;
                }
                iteration += 1;
            }
            count as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_pushes() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
    }

    #[test]
    fn test_minimum_pushes2() {
        assert_eq!(Solution::minimum_pushes("xycdefghij".to_string()), 12);
    }
}
