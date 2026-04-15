// Problem 3395: subsequences with a unique middle mode i
// #Hard #Array #Hash_Table #Math #Combinatorics

const MOD: i64 = 1_000_000_007;

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
        let c2 = |n: i64| -> i64 { if n < 2 { 0 } else { n * (n - 1) / 2 } };
        let n = nums.len();
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut new_nums = vec![0usize; n];
        let mut m = 0usize;
        for x in &nums {
            let id = if let Some(&id) = map.get(x) {
                id
            } else {
                let id = m;
                map.insert(*x, id);
                m += 1;
                id
            };
            new_nums[m - 1] = id;
        }
        // Recompute new_nums properly
        m = 0;
        map.clear();
        for (idx, x) in nums.iter().enumerate() {
            let id = if let Some(&id) = map.get(x) {
                id
            } else {
                let id = m;
                map.insert(*x, id);
                m += 1;
                id
            };
            new_nums[idx] = id;
        }
        if m == n {
            return 0;
        }
        let mut right_count = vec![0i64; m];
        for &x in &new_nums {
            right_count[x] += 1;
        }
        let mut left_count = vec![0i64; m];
        let mut ans: i64 = n as i64 * (n as i64 - 1) * (n as i64 - 2) * (n as i64 - 3) * (n as i64 - 4) / 120;
        for left in 0..n - 2 {
            let x = new_nums[left];
            right_count[x] -= 1;
            if left >= 2 {
                let right = (n - (left + 1)) as i64;
                let left_x = left_count[x];
                let right_x = right_count[x];
                ans -= c2(left as i64 - left_x) * c2(right - right_x);
                for y in 0..m {
                    if y == x {
                        continue;
                    }
                    let right_y = right_count[y];
                    let left_y = left_count[y];
                    ans -= c2(left_y) * right_x * (right - right_x);
                    ans -= c2(right_y) * left_x * (left as i64 - left_x);
                    ans -= left_y
                        * right_y
                        * (left_x * (right - right_x - right_y)
                            + right_x * (left as i64 - left_x - left_y));
                }
            }
            left_count[x] += 1;
        }
        (ans % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void subsequencesWithMiddleMode()
    //   assertThat(
    //   new Solution().subsequencesWithMiddleMode(new int[] {1, 1, 1, 1, 1, 1}),
    //   equalTo(6));
    #[test]
    fn test_subsequences_with_middle_mode() {
        // TODO: 翻译 Java 测试
    }

    // Java: void subsequencesWithMiddleMode2()
    //   assertThat(
    //   new Solution().subsequencesWithMiddleMode(new int[] {1, 2, 2, 3, 3, 4}),
    //   equalTo(4));
    #[test]
    fn test_subsequences_with_middle_mode2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void subsequencesWithMiddleMode3()
    //   assertThat(
    //   new Solution().subsequencesWithMiddleMode(new int[] {0, 1, 2, 3, 4, 5, 6, 7, 8}),
    //   equalTo(0));
    #[test]
    fn test_subsequences_with_middle_mode3() {
        // TODO: 翻译 Java 测试
    }
}
