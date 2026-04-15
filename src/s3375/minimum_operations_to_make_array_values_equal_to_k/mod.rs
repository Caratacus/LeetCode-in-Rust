// Problem 3375: Minimum Operations to Make Array Values Equal to K
// #Easy #Array #Hash_Table

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for &i in &nums {
            set.insert(i);
        }
        let mut res = 0;
        for &i in &set {
            if i > k {
                res += 1;
            } else if i < k {
                return -1;
            }
        }
        res
    }
}
