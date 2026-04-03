// Problem 1589: Maximum Sum Obtained of Any Permutation
// #Medium #Array #Sorting #Greedy #Prefix_Sum
// #Big_O_Time_O(n_log_n)_Space_O(n)

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_sum_range_query(mut nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        nums.sort_unstable();
        let l = nums.len();
        let mut temp_arr = vec![0i32; l];

        // Calculate frequency of each index using difference array technique
        for request in &requests {
            let a = request[0] as usize;
            let b = (request[1] + 1) as usize;
            temp_arr[a] += 1;
            if b < l {
                temp_arr[b] -= 1;
            }
        }

        // Compute prefix sum to get actual frequencies
        let mut prev = 0i32;
        for i in 0..l {
            temp_arr[i] += prev;
            prev = temp_arr[i];
        }

        // Sort frequencies
        temp_arr.sort_unstable();

        // Calculate answer by pairing highest frequencies with highest numbers
        let mut ans = 0i64;
        let mut index = l as i32 - 1;
        while index >= 0 {
            if temp_arr[index as usize] == 0 {
                break;
            }
            let x = (temp_arr[index as usize] as i64) % MOD;
            let y = (nums[index as usize] as i64) % MOD;
            index -= 1;
            ans = (ans + x * y) % MOD;
        }

        ans as i32
    }
}
