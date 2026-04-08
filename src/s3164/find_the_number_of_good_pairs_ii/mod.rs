// Problem 3164: find the number of good pairs ii
// #Medium #Array #Hash_Table #2024_06_02_Time_407_ms_(75.28%)_Space_66.8_MB_(7.30%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut hm: HashMap<i32, i64> = HashMap::new();
        for &val in &nums2 {
            *hm.entry(val * k).or_insert(0) += 1;
        }

        let mut ans: i64 = 0;
        for &i in &nums1 {
            if i % k != 0 {
                continue;
            }
            let mut factor = 1;
            while factor * factor <= i {
                if i % factor != 0 {
                    factor += 1;
                    continue;
                }
                let factor2 = i / factor;
                if let Some(&cnt) = hm.get(&factor) {
                    ans += cnt;
                }
                if factor != factor2 {
                    if let Some(&cnt) = hm.get(&factor2) {
                        ans += cnt;
                    }
                }
                factor += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_pairs() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1),
            5
        );
    }

    #[test]
    fn test_number_of_pairs2() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3),
            2
        );
    }
}
