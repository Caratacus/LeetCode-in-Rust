// Problem 3000: maximum area of longest diagonal rectangle
// #Easy #Array #2024_01_17_Time_1_ms_(99.67%)_Space_44.1_MB_(93.21%)

pub struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut mx = 0;
        for t in &dimensions {
            let diag = t[0] * t[0] + t[1] * t[1];
            if diag > mx {
                mx = diag;
            }
        }
        let mut area = 0;
        for t in &dimensions {
            let diag = t[0] * t[0] + t[1] * t[1];
            if diag == mx && t[0] * t[1] > area {
                area = t[0] * t[1];
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void areaOfMaxDiagonal()
    //   assertThat(new Solution().areaOfMaxDiagonal(new int[][] {{9, 3}, {8, 6}}), equalTo(48));
    #[test]
    fn test_area_of_max_diagonal() {
        assert_eq!(Solution::area_of_max_diagonal(vec![vec![9, 3], vec![8, 6]]), 48);
    }

    // Java: void areaOfMaxDiagonal2()
    //   assertThat(new Solution().areaOfMaxDiagonal(new int[][] {{3, 4}, {4, 3}}), equalTo(12));
    #[test]
    fn test_area_of_max_diagonal2() {
        assert_eq!(Solution::area_of_max_diagonal(vec![vec![3, 4], vec![4, 3]]), 12);
    }
}
