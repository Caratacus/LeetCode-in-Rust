// Problem 3011: find if array can be sorted
// #Medium #Array #Sorting #Bit_Manipulation

pub struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut last_group_max = i32::MIN;
        let mut max = nums[0];
        let mut last_bit = nums[0].count_ones();
        for i in 1..nums.len() {
            let bit = nums[i].count_ones();
            if bit == last_bit {
                max = max.max(nums[i]);
            } else {
                last_group_max = max;
                max = nums[i];
                last_bit = bit;
            }
            if nums[i] < last_group_max {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_sort_array() {
        assert_eq!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]), true);
    }

    #[test]
    fn test_can_sort_array2() {
        assert_eq!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn test_can_sort_array3() {
        assert_eq!(Solution::can_sort_array(vec![3, 16, 8, 4, 2]), false);
    }
}
