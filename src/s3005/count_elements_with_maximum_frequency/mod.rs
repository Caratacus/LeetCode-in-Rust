// Problem 3005: count elements with maximum frequency
// #Easy #Array #Hash_Table #Counting #2024_02_26_Time_1_ms_(99.76%)_Space_41.6_MB_(98.97%)

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut seen: HashSet<i32> = HashSet::new();
        let mut co = 0i32;
        let mut prev = 0i32;
        for &num in &nums {
            if seen.contains(&num) {
                continue;
            }
            seen.insert(num);
            if seen.len() == nums.len() {
                break;
            }
            let mut c = 0i32;
            for &i in &nums {
                if num == i {
                    c += 1;
                }
            }
            if c > 1 {
                if c > prev {
                    co = c;
                    prev = c;
                } else if c == prev {
                    co = c + co;
                }
            }
        }
        if co == 0 {
            nums.len() as i32
        } else {
            co
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_frequency_elements() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    }

    #[test]
    fn test_max_frequency_elements2() {
        assert_eq!(Solution::max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
