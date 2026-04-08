// Problem 3033: modify the matrix
// #Easy #Array #Matrix #2024_03_01_Time_1_ms_(100.00%)_Space_45.4_MB_(77.37%)

pub struct Solution;

impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let cols = matrix[0].len();
        for i in 0..matrix.len() {
            for j in 0..cols {
                if matrix[i][j] == -1 {
                    let mut max_val = 0;
                    for row in &matrix {
                        if row[j] > max_val {
                            max_val = row[j];
                        }
                    }
                    matrix[i][j] = max_val;
                }
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modified_matrix() {
        assert_eq!(
            Solution::modified_matrix(vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]]),
            vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]]
        );
    }

    #[test]
    fn test_modified_matrix2() {
        assert_eq!(
            Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]]),
            vec![vec![3, 2], vec![5, 2]]
        );
    }
}
