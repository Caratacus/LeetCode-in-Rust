// Problem 3131: find the integer added to array i
// #Easy #Array #2024_04_02_Time_0 ms(100.00%)  Space 43 MB (75.29%)

pub struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let mut s1 = 0;
        let mut s2 = 0;

        for i in nums1 {
            s1 += i;
        }
        for i in nums2 {
            s2 += i;
        }

        (s2 - s1) / n1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_added_integer() {
        assert_eq!(Solution::added_integer(vec![2, 6, 4], vec![9, 7, 5]), 3);
    }

    #[test]
    fn test_added_integer2() {
        assert_eq!(Solution::added_integer(vec![10], vec![5]), -5);
    }

    #[test]
    fn test_added_integer3() {
        assert_eq!(Solution::added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), 0);
    }
}
