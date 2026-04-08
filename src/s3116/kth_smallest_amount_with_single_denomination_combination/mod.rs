// Problem 3116: Kth Smallest Amount with Single Denomination Combination
// #Hard #Array #Math #Binary_Search #Bit_Manipulation #Number_Theory #Combinatorics
// #2024_04_27_Time_2_ms_(100.00%)_Space_41.4_MB_(72.21%)

pub struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let min_c = *coins.iter().min().unwrap();
        let cc = Self::coins(&coins);
        let upper = (min_c as i64) * (k as i64);
        let mut lo = upper / coins.len() as i64;
        let mut hi = upper;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let cnt = Self::count(&cc, mid);
            if cnt > k as i64 {
                hi = mid - 1;
            } else if cnt < k as i64 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn count(coins: &[i64], v: i64) -> i64 {
        let mut r: i64 = 0;
        for &c in coins {
            r += v / c;
        }
        r
    }

    fn coins(coins: &[i32]) -> Vec<i64> {
        let mut coins = coins.to_vec();
        coins.sort();
        let mut len = 1;
        'outer: for i in 1..coins.len() {
            let c = coins[i];
            for j in 0..len {
                if c % coins[j] == 0 {
                    continue 'outer;
                }
            }
            coins[len] = c;
            len += 1;
        }
        let coins = &coins[..len];
        let mut res = vec![0i64; (1 << coins.len()) - 1];
        Self::iterate(coins, &mut res, 1, 0, 0, true);
        res
    }

    fn iterate(
        coins: &[i32],
        res: &mut Vec<i64>,
        mult: i64,
        start: usize,
        mut idx: usize,
        positive: bool,
    ) -> usize {
        for i in start..coins.len() {
            let next = mult * coins[i] as i64 / Self::gcd(mult, coins[i] as i64);
            res[idx] = if positive { next } else { -next };
            idx += 1;
            idx = Self::iterate(coins, res, next, i + 1, idx, !positive);
        }
        idx
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void findKthSmallest()
    //   assertThat(new Solution().findKthSmallest(new int[] {3, 6, 9}, 3), equalTo(9L));
    #[test]
    fn test_find_kth_smallest() {
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
    }

    // Java: void findKthSmallest2()
    //   assertThat(new Solution().findKthSmallest(new int[] {5, 2}, 7), equalTo(12L));
    #[test]
    fn test_find_kth_smallest2() {
        assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
    }
}
