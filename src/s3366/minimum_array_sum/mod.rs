// Problem 3366: Minimum Array Sum
// #Medium #Array #Dynamic_Programming

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32, op1: i32, op2: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let high = Self::lower_bound(&nums, k * 2 - 1);
        let low = Self::lower_bound(&nums, k);
        let mut op1 = op1;
        let mut op2 = op2;

        // Process high values (>= 2k-1) from high to end, in reverse order
        for i in (high..n).rev() {
            if op1 > 0 {
                nums[i] = (nums[i] + 1) / 2;
                op1 -= 1;
            }
            if op2 > 0 {
                nums[i] -= k;
                op2 -= 1;
            }
        }

        // Process middle values [low, high)
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut odd = 0;
        for i in low..high {
            if op2 > 0 {
                nums[i] -= k;
                if k % 2 > 0 && nums[i] % 2 > 0 {
                    *count.entry(nums[i]).or_insert(0) += 1;
                }
                op2 -= 1;
            } else {
                odd += nums[i] % 2;
            }
        }

        // Sort the lower part [0, high)
        nums[..high].sort();
        let mut ans = 0;

        // Apply op1 optimization for k odd
        if k % 2 > 0 {
            for i in (high as i32 - op1) as usize..high {
                if odd == 0 {
                    break;
                }
                let x = nums[i];
                if let Some(c) = count.get_mut(&x) {
                    if *c > 0 {
                        *c -= 1;
                        if *c == 0 {
                            count.remove(&x);
                        }
                        odd -= 1;
                        ans -= 1;
                    }
                }
            }
        }

        // Apply remaining op1 from high-1 downward
        let mut idx = high as i32 - 1;
        let mut remaining_op1 = op1;
        while idx >= 0 && remaining_op1 > 0 {
            nums[idx as usize] = (nums[idx as usize] + 1) / 2;
            idx -= 1;
            remaining_op1 -= 1;
        }

        // Calculate sum
        for x in &nums {
            ans += x;
        }
        ans
    }

    fn lower_bound(nums: &[i32], target: i32) -> usize {
        let mut left = -1i32;
        let mut right = nums.len() as i32;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if nums[mid as usize] >= target {
                right = mid;
            } else {
                left = mid;
            }
        }
        right as usize
    }
}
