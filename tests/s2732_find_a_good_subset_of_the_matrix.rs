// Tests for Problem 2732: Find a Good Subset of the Matrix
// Java reference: src/test/java/g2701_2800/s2732_find_a_good_subset_of_the_matrix/SolutionTest.java

use leetcode_in_rust::s2732::find_a_good_subset_of_the_matrix::Solution;

#[test]
fn test_good_subsetof_binary_matrix() {
    assert_eq!(
        Solution::good_subsetof_binary_matrix(vec![vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 1, 1, 1]]),
        vec![0, 1]
    );
}

#[test]
fn test_good_subsetof_binary_matrix2() {
    assert_eq!(
        Solution::good_subsetof_binary_matrix(vec![vec![0]]),
        vec![0]
    );
}

#[test]
fn test_good_subsetof_binary_matrix3() {
    assert_eq!(
        Solution::good_subsetof_binary_matrix(vec![vec![1, 1, 0], vec![1, 1, 0]]),
        vec![] as Vec<i32>
    );
}
