// Problem 3012: minimize length of array using operations
// #Medium #Array #Math #Greedy #Number_Theory

pub struct Solution;

impl Solution {
    pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for &i in &nums {
            if i < min {
                min = i;
            }
            if i > max {
                max = i;
            }
        }
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        if max % min != 0 {
            return 1;
        }
        let mut count = 0;
        for &i in &nums {
            if i % min != 0 && i % min < min {
                return 1;
            }
            if i == min {
                count += 1;
            }
        }
        (count + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_array_length() {
        assert_eq!(Solution::minimum_array_length(vec![1, 4, 3, 1]), 1);
    }

    #[test]
    fn test_minimum_array_length2() {
        assert_eq!(Solution::minimum_array_length(vec![5, 5, 5, 10, 5]), 2);
    }

    #[test]
    fn test_minimum_array_length3() {
        assert_eq!(Solution::minimum_array_length(vec![2, 3, 4]), 1);
    }
}
