// Problem 3404: count special subsequences
// #Medium #Array #Hash_Table #Math #Enumeration

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let mut freq: HashMap<u64, i32> = HashMap::new();
        let mut ans: i64 = 0;
        let n = nums.len();
        for r in 4..n {
            for p in 0..(r - 2) {
                let q = r - 2;
                if p >= q - 1 {
                    break;
                }
                // Use fraction as key to avoid float issues: nums[p]/nums[q] stored as ordered pair
                let key = Self::make_key(nums[p], nums[q]);
                *freq.entry(key).or_insert(0) += 1;
            }
            for s in (r + 2)..n {
                let key = Self::make_key(nums[s], nums[r]);
                ans += freq.get(&key).copied().unwrap_or(0) as i64;
            }
        }
        ans
    }

    fn make_key(a: i32, b: i32) -> u64 {
        // Store as (a, b) using Rational number reduction for exact comparison
        if b == 0 {
            return (a as u64).wrapping_shl(32);
        }
        let g = gcd(a.unsigned_abs(), b.unsigned_abs());
        let a_reduced = (a / g as i32) as u32 as u64;
        let b_reduced = (b / g as i32) as u32 as u64;
        a_reduced.wrapping_shl(32) | b_reduced
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numberOfSubsequences()
    //   assertThat(
    //   new Solution().numberOfSubsequences(new int[] {1, 2, 3, 4, 3, 6, 1}), equalTo(1L));
    #[test]
    fn test_number_of_subsequences() {
        // TODO: 翻译 Java 测试
    }

    // Java: void numberOfSubsequences2()
    //   assertThat(
    //   new Solution().numberOfSubsequences(new int[] {3, 4, 3, 4, 3, 4, 3, 4}),
    //   equalTo(3L));
    #[test]
    fn test_number_of_subsequences2() {
        // TODO: 翻译 Java 测试
    }
}
