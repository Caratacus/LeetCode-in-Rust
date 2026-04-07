// Tests for Problem 1975: Maximum Matrix Sum
// Java reference: src/test/java/g1901_2000/s1975_maximum_matrix_sum/SolutionTest.java

use leetcode_in_rust::s1975::maximum_matrix_sum::Solution;

#[test]
fn test_max_matrix_sum() {
    assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
}

#[test]
fn test_max_matrix_sum2() {
    assert_eq!(
        Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
        16
    );
}
