// Problem 3002: maximum size of a set after removals
// #Medium #Array #Hash_Table #Greedy #2024_02_26_Time_26_ms_(94.01%)_Space_53.3_MB_(80.90%)

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn maximum_set_size(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut uniq1: HashSet<i32> = HashSet::new();
        let mut uniq2: HashSet<i32> = HashSet::new();
        for i in 0..nums1.len() {
            uniq1.insert(nums1[i]);
            uniq2.insert(nums2[i]);
        }
        let common = if uniq1.len() <= uniq2.len() {
            uniq1.iter().filter(|&u| uniq2.contains(u)).count() as i32
        } else {
            uniq2.iter().filter(|&u| uniq1.contains(u)).count() as i32
        };
        let half = (nums1.len() / 2) as i32;
        let from1 = std::cmp::min(uniq1.len() as i32 - common, half);
        let from2 = std::cmp::min(uniq2.len() as i32 - common, half);
        let take_from_common1 = half - from1;
        let take_from_common2 = half - from2;
        from1 + from2 + std::cmp::min(take_from_common1 + take_from_common2, common)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_set_size() {
        assert_eq!(
            Solution::maximum_set_size(vec![1, 2, 1, 2], vec![1, 1, 1, 1]),
            2
        );
    }

    #[test]
    fn test_maximum_set_size2() {
        assert_eq!(
            Solution::maximum_set_size(vec![1, 2, 3, 4, 5, 6], vec![2, 3, 2, 3, 2, 3]),
            5
        );
    }

    #[test]
    fn test_maximum_set_size3() {
        assert_eq!(
            Solution::maximum_set_size(vec![1, 1, 2, 2, 3, 3], vec![4, 4, 5, 5, 6, 6]),
            6
        );
    }
}
