// Tests for Problem 2872: Maximum Number of K-Divisible Components
// Java reference: src/test/java/g2801_2900/s2872_maximum_number_of_k_divisible_components/SolutionTest.java

use leetcode_in_rust::s2872::maximum_number_of_k_divisible_components::Solution;

#[test]
fn test_max_k_divisible_components() {
    assert_eq!(
        Solution::max_k_divisible_components(
            5,
            vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            vec![1, 8, 1, 4, 4],
            6
        ),
        2
    );
}

#[test]
fn test_max_k_divisible_components2() {
    assert_eq!(
        Solution::max_k_divisible_components(
            7,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]],
            vec![3, 0, 6, 1, 5, 2, 1],
            3
        ),
        3
    );
}
