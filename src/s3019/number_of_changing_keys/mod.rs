// Problem 3019: number of changing keys
// #Easy #String #Breadth_First_Search #Graph #Prefix_Sum

pub struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let s = s.to_lowercase();
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;

        for i in 0..chars.len() - 1 {
            if chars[i] != chars[i + 1] {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_key_changes() {
        assert_eq!(Solution::count_key_changes("aAbBcC".to_string()), 2);
    }

    #[test]
    fn test_count_key_changes2() {
        assert_eq!(Solution::count_key_changes("AaAaAaaA".to_string()), 0);
    }
}
