// Problem 3139: minimum cost to equalize array
// #Hard #Array #Greedy #Enumeration #2024_05_07_Time_1_ms_(100.00%)_Space_57.2_MB_(83.16%)

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn min_cost_to_equalize_array(nums: Vec<i32>, cost1: i32, cost2: i32) -> i32 {
        let mut max: i64 = 0;
        let mut min: i64 = i64::MAX;
        let mut sum: i64 = 0;
        for &num in &nums {
            let num = num as i64;
            if num > max {
                max = num;
            }
            if num < min {
                min = num;
            }
            sum += num;
        }
        let n = nums.len() as i64;
        let total = max * n - sum;
        // When operation one is always better:
        if (cost1 as i64) * 2 <= cost2 as i64 || n <= 2 {
            return ((total * cost1 as i64) % MOD) as i32;
        }
        // When operation two is moderately better:
        let op1 = ((max - min) * 2 - total).max(0);
        let op2 = total - op1;
        let mut result = (op1 + (op2 & 1)) * cost1 as i64 + (op2 >> 1) * cost2 as i64;
        // When operation two is significantly better:
        let mut total = total + op1 / (n - 2) * n;
        let op1 = op1 % (n - 2);
        let op2 = total - op1;
        result = result.min((op1 + (op2 & 1)) * cost1 as i64 + (op2 >> 1) * cost2 as i64);
        // When operation two is always better:
        for _ in 0..2 {
            total += n;
            result = result.min((total & 1) * cost1 as i64 + (total >> 1) * cost2 as i64);
        }
        (result % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_to_equalize_array() {
        assert_eq!(Solution::min_cost_to_equalize_array(vec![4, 1], 5, 2), 15);
    }

    #[test]
    fn test_min_cost_to_equalize_array2() {
        assert_eq!(Solution::min_cost_to_equalize_array(vec![2, 3, 3, 3, 5], 2, 1), 6);
    }

    #[test]
    fn test_min_cost_to_equalize_array3() {
        assert_eq!(Solution::min_cost_to_equalize_array(vec![3, 5, 3], 1, 3), 4);
    }
}
