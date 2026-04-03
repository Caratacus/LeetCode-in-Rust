// Problem 1590: Make Sum Divisible by P
// #Medium #Array #Hash_Table #Prefix_Sum
// #Big_O_Time_O(n)_Space_O(n)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut hmp: HashMap<i32, i32> = HashMap::new();
        let n = nums.len();
        let mut target = 0i32;
        let mut sum = 0i32;

        for &num in &nums {
            target = (num + target) % p;
        }

        if target == 0 {
            return 0;
        }

        hmp.insert(0, -1);
        let mut ans = n as i32;

        for i in 0..n {
            sum = (sum + nums[i]) % p;
            let key = (sum - target + p) % p;
            if let Some(&val) = hmp.get(&key) {
                ans = ans.min(i as i32 - val);
            }
            hmp.insert(sum % p, i as i32);
        }

        if ans < n as i32 {
            ans
        } else {
            -1
        }
    }
}
