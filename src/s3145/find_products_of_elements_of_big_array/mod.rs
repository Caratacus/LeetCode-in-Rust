// Problem 3145: find products of elements of big array
// #Hard #Array #Binary_Search #Bit_Manipulation
// #2024_05_15_Time_3_ms_(98.41%)_Space_44.5_MB_(96.83%)

pub struct Solution;

impl Solution {
    pub fn find_products_of_elements(queries: Vec<Vec<i64>>) -> Vec<i32> {
        queries
            .iter()
            .map(|q| {
                let er = Self::sum_e(q[1] + 1);
                let el = Self::sum_e(q[0]);
                Self::pow(2, er - el, q[2])
            })
            .collect()
    }

    fn sum_e(k: i64) -> i64 {
        let mut res = 0;
        let mut n: i64 = 0;
        let mut cnt1: i64 = 0;
        let mut sum_i: i64 = 0;
        let mut k = k;

        let leading_zeros = if k + 1 == 0 { 64 } else { (k + 1).leading_zeros() };
        for i in (1..=63 - leading_zeros as i64).rev() {
            let c = (cnt1 << i) + (i << (i - 1));
            if c <= k {
                k -= c;
                res += (sum_i << i) + ((i * (i - 1) / 2) << (i - 1));
                sum_i += i;
                cnt1 += 1;
                n |= 1 << i;
            }
        }
        if cnt1 <= k {
            k -= cnt1;
            res += sum_i;
            n += 1;
        }
        while k > 0 {
            k -= 1;
            res += n.trailing_zeros() as i64;
            n &= n - 1;
        }
        res
    }

    fn pow(x: i64, n: i64, modulo: i64) -> i32 {
        let mut res = 1 % modulo;
        let mut x = x % modulo;
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 {
                res = (res * x) % modulo;
            }
            x = (x * x) % modulo;
            n /= 2;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_products_of_elements() {
        assert_eq!(
            Solution::find_products_of_elements(vec![vec![1, 3, 7]]),
            vec![4]
        );
    }

    #[test]
    fn test_find_products_of_elements2() {
        assert_eq!(
            Solution::find_products_of_elements(vec![vec![2, 5, 3], vec![7, 7, 4]]),
            vec![2, 2]
        );
    }
}
