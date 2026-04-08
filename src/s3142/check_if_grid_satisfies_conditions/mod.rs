// Problem 3142: check if grid satisfies conditions
// #Easy #Array #Matrix #2024_05_15_Time_1_ms_(95.76%)_Space_44.4_MB_(59.70%)

pub struct Solution;

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m - 1 {
            if n > 1 {
                for j in 0..n - 1 {
                    if grid[i][j] != grid[i + 1][j] || grid[i][j] == grid[i][j + 1] {
                        return false;
                    }
                }
            } else {
                if grid[i][0] != grid[i + 1][0] {
                    return false;
                }
            }
        }
        for j in 0..n - 1 {
            if grid[m - 1][j] == grid[m - 1][j + 1] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfies_conditions() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![1, 0, 2], vec![1, 0, 2]]), true);
    }

    #[test]
    fn test_satisfies_conditions2() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![1, 1, 1], vec![0, 0, 0]]), false);
    }

    #[test]
    fn test_satisfies_conditions3() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![1], vec![2], vec![3]]), false);
    }

    #[test]
    fn test_satisfies_conditions4() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![1], vec![1]]), true);
    }

    #[test]
    fn test_satisfies_conditions5() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![1, 2, 3]]), true);
    }

    #[test]
    fn test_satisfies_conditions6() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![1, 1]]), false);
    }

    #[test]
    fn test_satisfies_conditions7() {
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![1, 2, 2], vec![3, 4, 5]]),
            false
        );
    }

    #[test]
    fn test_satisfies_conditions8() {
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]),
            true
        );
    }

    #[test]
    fn test_satisfies_conditions9() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![5, 1], vec![5, 0]]), true);
    }

    #[test]
    fn test_satisfies_conditions10() {
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![1, 0], vec![2, 2]]),
            false
        );
    }

    #[test]
    fn test_satisfies_conditions11() {
        assert_eq!(Solution::satisfies_conditions(vec![vec![7]]), true);
    }

    #[test]
    fn test_satisfies_conditions12() {
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![4, 1, 5, 2], vec![3, 0, 4, 1]]),
            false
        );
    }

    #[test]
    fn test_satisfies_conditions13() {
        assert_eq!(
            Solution::satisfies_conditions(vec![vec![2, 3, 3, 1], vec![1, 0, 4, 2]]),
            false
        );
    }
}
