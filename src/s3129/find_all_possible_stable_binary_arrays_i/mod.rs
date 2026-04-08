// Problem 3129: find all possible stable binary arrays i
// #Medium #Dynamic_Programming #Prefix_Sum #2024_05_02_Time_3_ms_(100.00%)_Space_44.1_MB_(98.38%)

const MODULUS: i64 = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        if limit == 1 {
            return (2 - (zero - one).abs()).max(0);
        }

        let max = zero.max(one) as usize;
        let min = zero.min(one) as usize;
        let mut lcn: Vec<Vec<i64>> = vec![vec![0; max + 1]; max + 1];
        lcn[0][0] = 1;

        for s in 1..=max {
            let s_lim = s as i32 - limit;
            let row2 = if s_lim > 0 { lcn[s_lim as usize - 1].clone() } else { vec![] };
            let row1 = lcn[s - 1].clone();
            for c in 1..=s {
                if c <= s_lim as usize {
                    lcn[s][c] = (row1[c] + row1[c - 1] + MODULUS - row2[c - 1]) % MODULUS;
                } else {
                    lcn[s][c] = (row1[c] + row1[c - 1]) % MODULUS;
                }
            }
        }

        let row1 = &lcn[min];
        let row0 = &lcn[max];
        let mut result = 0i64;
        let mut s0 = (if min < max { row0[min + 1] } else { 0 } + row0[min]) % MODULUS;
        for c in (1..=min).rev() {
            let s1 = s0;
            s0 = (row0[c] + row0[c - 1]) % MODULUS;
            result = (result + (row1[c] * (s0 + s1) % MODULUS)) % MODULUS;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_stable_arrays() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2);
    }

    #[test]
    fn test_number_of_stable_arrays2() {
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1);
    }

    #[test]
    fn test_number_of_stable_arrays3() {
        assert_eq!(Solution::number_of_stable_arrays(3, 3, 2), 14);
    }
}
