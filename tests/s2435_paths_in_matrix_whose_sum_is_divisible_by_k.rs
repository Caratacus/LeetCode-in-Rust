// Tests for Problem 2435: Paths in Matrix Whose Sum Is Divisible by K
// Java reference: src/test/java/g2401_2500/s2435_paths_in_matrix_whose_sum_is_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s2435::paths_in_matrix_whose_sum_is_divisible_by_k::Solution;

#[test]
fn test_number_of_paths() {
    assert_eq!(
        Solution::number_of_paths(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3),
        2
    );
}

#[test]
fn test_number_of_paths2() {
    assert_eq!(Solution::number_of_paths(vec![vec![0, 0]], 5), 1);
}

#[test]
fn test_number_of_paths3() {
    assert_eq!(
        Solution::number_of_paths(
            vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]],
            1
        ),
        10
    );
}
