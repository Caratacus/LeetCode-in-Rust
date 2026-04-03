// Problem 1567: Maximum Length of Subarray With Positive Product
// #Medium #Array #Dynamic_Programming #Greedy
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        let mut pos_len = 0i32;
        let mut neg_len = 0i32;
        let mut res = 0i32;

        for num in nums {
            if num == 0 {
                pos_len = 0;
                neg_len = 0;
            } else if num > 0 {
                pos_len += 1;
                neg_len = if neg_len == 0 { 0 } else { neg_len + 1 };
            } else {
                let temp = pos_len;
                pos_len = if neg_len == 0 { 0 } else { neg_len + 1 };
                neg_len = temp + 1;
            }
            res = res.max(pos_len);
        }
        res
    }
}
