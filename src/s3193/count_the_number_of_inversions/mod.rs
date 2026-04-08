// Problem 3193: count the number of inversions
// #Hard #Array #Dynamic_Programming #2024_06_26_Time_11_ms_(96.54%)_Space_45.5_MB_(37.54%)

pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_permutations(n: i32, r: Vec<Vec<i32>>) -> i32 {
        let mut r = r;
        r.sort_by_key(|x| x[0]);
        if r[0][0] == 0 && r[0][1] > 0 {
            return 0;
        }
        let n = n as usize;
        let mut ri = if r[0][0] == 0 { 1 } else { 0 };
        let mut a: i64 = 1;
        let m = vec![vec![0i32; 401]; n];
        let mut m = m;
        m[0][0] = 1;
        for i in 1..n {
            m[i][0] = m[i - 1][0];
            for j in 1..=i {
                m[i][j] = (m[i][j] + m[i][j - 1]) % MOD as i32;
                m[i][j] = (m[i][j] + m[i - 1][j]) % MOD as i32;
            }
            for j in (i + 1)..=r[ri][1] as usize {
                m[i][j] = (m[i][j] + m[i][j - 1]) % MOD as i32;
                m[i][j] = (m[i][j] + m[i - 1][j]) % MOD as i32;
                m[i][j] = m[i][j] - m[i - 1][j - i - 1];
                if m[i][j] < 0 {
                    m[i][j] += MOD as i32;
                }
            }
            if r[ri][0] as usize == i {
                let t = m[i][r[ri][1] as usize];
                if t == 0 {
                    return 0;
                }
                for j in 0..=r[ri][1] as usize {
                    m[i][j] = 0;
                }
                m[i][r[ri][1] as usize] = 1;
                a = (a * t as i64) % MOD;
                ri += 1;
            }
        }
        a as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numberOfPermutations()
    //   assertThat(
    //   new Solution().numberOfPermutations(3, new int[][] {{2, 2}, {0, 0}}), equalTo(2));
    #[test]
    fn test_number_of_permutations() {
        assert_eq!(
            Solution::number_of_permutations(3, vec![vec![2, 2], vec![0, 0]]),
            2
        );
    }

    // Java: void numberOfPermutations2()
    //   assertThat(
    //   new Solution().numberOfPermutations(3, new int[][] {{2, 2}, {1, 1}, {0, 0}}),
    //   equalTo(1));
    #[test]
    fn test_number_of_permutations2() {
        assert_eq!(
            Solution::number_of_permutations(3, vec![vec![2, 2], vec![1, 1], vec![0, 0]]),
            1
        );
    }

    // Java: void numberOfPermutations3()
    //   assertThat(
    //   new Solution().numberOfPermutations(2, new int[][] {{0, 0}, {1, 0}}), equalTo(1));
    #[test]
    fn test_number_of_permutations3() {
        assert_eq!(
            Solution::number_of_permutations(2, vec![vec![0, 0], vec![1, 0]]),
            1
        );
    }
}
