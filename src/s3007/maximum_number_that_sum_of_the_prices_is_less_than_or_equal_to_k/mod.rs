// Problem 3007: maximum number that sum of the prices is less than or equal to k
// #Medium #Dynamic_Programming #Binary_Search #Bit_Manipulation
// #2024_02_26_Time_1_ms_(100.00%)_Space_41_MB_(47.71%)

pub struct Solution;

impl Solution {
    fn count(k: i64, bit: i64, x: i32) -> i64 {
        if k < bit {
            return 0;
        }
        let mut n: i64 = 1;
        let mut bits = bit;
        let mut p: i64 = 1;
        while 2 * bits + (if p % (x as i64) == 0 { n } else { 0 }) <= k {
            bits = 2 * bits + (if p % (x as i64) == 0 { n } else { 0 });
            n *= 2;
            p += 1;
        }
        n + Self::count(k - bits, bit + (if p % (x as i64) == 0 { 1 } else { 0 }), x)
    }

    pub fn find_maximum_number(k: i64, x: i32) -> i64 {
        Self::count(k, 0, x) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_number() {
        assert_eq!(Solution::find_maximum_number(9, 1), 6);
    }

    #[test]
    fn test_find_maximum_number2() {
        assert_eq!(Solution::find_maximum_number(7, 2), 9);
    }
}
