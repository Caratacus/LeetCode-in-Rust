// Tests for Problem 2813: Maximum Elegance of a K Length Subsequence
// Java reference: src/test/java/g2801_2900/s2813_maximum_elegance_of_a_k_length_subsequence/SolutionTest.java

use leetcode_in_rust::s2813::maximum_elegance_of_a_k_length_subsequence::Solution;

#[test]
fn test_find_maximum_elegance() {
    assert_eq!(
        Solution::find_maximum_elegance(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2),
        17
    );
}

#[test]
fn test_find_maximum_elegance2() {
    assert_eq!(
        Solution::find_maximum_elegance(vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]], 3),
        19
    );
}

#[test]
fn test_find_maximum_elegance3() {
    assert_eq!(
        Solution::find_maximum_elegance(vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3),
        7
    );
}
