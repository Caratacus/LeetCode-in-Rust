// Problem 3162: find the number of good pairs i
// #Easy #Array #Hash_Table #2024_05_30_Time_1_ms_(99.96%)_Space_42.1_MB_(99.36%)

pub struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for &n1 in &nums1 {
            for &n2 in &nums2 {
                if n1 % (n2 * k) == 0 {
                    count += 1;
                }
            }
        }
        count
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
