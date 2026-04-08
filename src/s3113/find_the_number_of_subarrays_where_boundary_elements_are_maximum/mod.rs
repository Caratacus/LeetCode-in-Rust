// Problem 3113: find the number of subarrays where boundary elements are maximum
// #Hard #Array #Binary_Search #Stack #Monotonic_Stack
// #2024_04_27_Time_13_ms_(98.83%)_Space_60.4_MB_(67.66%)

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut stack: VecDeque<(i32, i64)> = VecDeque::new();
        let mut res: i64 = 0;
        for a in nums {
            while !stack.is_empty() && stack.front().unwrap().0 < a {
                stack.pop_front();
            }
            if stack.is_empty() || stack.front().unwrap().0 != a {
                stack.push_front((a, 0));
            }
            let count = &mut stack.front_mut().unwrap().1;
            *count += 1;
            res += *count;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_subarrays() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 4, 3, 3, 2]), 6);
    }

    #[test]
    fn test_number_of_subarrays2() {
        assert_eq!(Solution::number_of_subarrays(vec![3, 3, 3]), 6);
    }

    #[test]
    fn test_number_of_subarrays3() {
        assert_eq!(Solution::number_of_subarrays(vec![1]), 1);
    }
}
