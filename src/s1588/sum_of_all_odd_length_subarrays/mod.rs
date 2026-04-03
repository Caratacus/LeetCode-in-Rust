// Problem 1588: Sum of All Odd Length Subarrays
// #Easy #Array #Math #Prefix_Sum
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let len = arr.len() as i32;
        let mut sum = 0i32;

        for i in 0..len {
            // Each element appears in ((i+1) * (len-i) + 1) / 2 subarrays of odd length
            sum += (((i + 1) * (len - i) + 1) / 2) * arr[i as usize];
        }

        sum
    }
}
