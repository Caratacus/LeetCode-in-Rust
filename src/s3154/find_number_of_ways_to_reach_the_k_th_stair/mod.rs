// Problem 3154: find number of ways to reach the k th stair
// #Hard #Dynamic_Programming #Math #Bit_Manipulation #Memoization #Combinatorics
// #2024_05_22_Time_0_ms_(100.00%)_Space_40.3_MB_(98.50%)

pub struct Solution;

impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let mut x: i64 = 1;
        let mut y: i64 = 1;
        let mut a: i32 = 0;
        while x > 0 && x - y <= k as i64 {
            if x >= k as i64 {
                a += Self::combi(y, x - k as i64);
            }
            x <<= 1;
            y += 1;
        }
        a
    }

    fn combi(a: i64, b: i64) -> i32 {
        let mut b = b;
        if b > a - b {
            b = a - b;
        }
        let mut r: i64 = 1;
        for i in 0..b {
            r *= a - i;
            r /= i + 1;
        }
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void waysToReachStair()
    //   assertThat(new Solution().waysToReachStair(0), equalTo(2));
    #[test]
    fn test_ways_to_reach_stair() {
        assert_eq!(Solution::ways_to_reach_stair(0), 2);
    }

    // Java: void waysToReachStair2()
    //   assertThat(new Solution().waysToReachStair(1), equalTo(4));
    #[test]
    fn test_ways_to_reach_stair2() {
        assert_eq!(Solution::ways_to_reach_stair(1), 4);
    }
}
