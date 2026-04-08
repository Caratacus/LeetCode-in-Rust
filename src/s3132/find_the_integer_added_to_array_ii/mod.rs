// Problem 3132: find the integer added to array ii
// #Medium #Array #Sorting #Hash_Table #2024_04_27_Time_O(n log n)_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        let nums2 = nums2;
        nums1.sort();
        let mut nums2_sorted = nums2.clone();
        nums2_sorted.sort();

        // Try x = nums2[0] - nums1[i] for i in 0, 1, 2
        for i in 0..3 {
            let x = nums2_sorted[0] - nums1[i];
            if Self::check_ok(&nums1, &nums2, x) {
                return x;
            }
        }
        // Try x = nums2[1] - nums1[i] for i in 0, 1, 2
        for i in 0..3 {
            let x = nums2_sorted[1] - nums1[i];
            if Self::check_ok(&nums1, &nums2, x) {
                return x;
            }
        }
        // Try x = nums2[2] - nums1[i] for i in 0, 1, 2
        for i in 0..3 {
            let x = nums2_sorted[2] - nums1[i];
            if Self::check_ok(&nums1, &nums2, x) {
                return x;
            }
        }

        nums2_sorted[0] - nums1[0]
    }

    fn check_ok(nums1: &[i32], nums2: &[i32], x: i32) -> bool {
        let mut i = 0;
        for &num in nums2 {
            while i < nums1.len() && nums1[i] + x < num {
                i += 1;
            }
            if i >= nums1.len() || nums1[i] + x != num {
                return false;
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_added_integer() {
        assert_eq!(
            Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]),
            -2
        );
    }

    #[test]
    fn test_minimum_added_integer2() {
        assert_eq!(Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7]), 2);
    }

    #[test]
    fn test_minimum_added_integer3() {
        assert_eq!(
            Solution::minimum_added_integer(
                vec![4, 6, 3, 1, 4, 2, 1, 8, 5, 2],
                vec![1, 8, 6, 3, 4, 5, 2]
            ),
            0
        );
    }
}
