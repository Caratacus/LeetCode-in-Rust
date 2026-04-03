// Tests for Problem 1594: Maximum Non Negative Product in a Matrix
// Java reference: src/test/java/g1501_1600/s1594_maximum_non_negative_product_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s1594::maximum_non_negative_product_in_a_matrix::Solution;

#[test]
fn test_max_product_path() {
    assert_eq!(
        Solution::max_product_path(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]]),
        -1
    );
}

#[test]
fn test_max_product_path2() {
    assert_eq!(
        Solution::max_product_path(vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]]),
        8
    );
}

#[test]
fn test_max_product_path3() {
    assert_eq!(Solution::max_product_path(vec![vec![1, 3], vec![0, -4]]), 0);
}
