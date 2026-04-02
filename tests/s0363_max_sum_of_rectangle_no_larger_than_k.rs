// Tests for Problem 0363: Max Sum of Rectangle No Larger Than K
// Java reference: src/test/java/g0301_0400/s0363_max_sum_of_rectangle_no_larger_than_k/SolutionTest.java

use leetcode_in_rust::s0363::max_sum_of_rectangle_no_larger_than_k::Solution;

#[test]
fn test_max_sum_submatrix() {
    assert_eq!(Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2), 2);
}

#[test]
fn test_max_sum_submatrix2() {
    assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);
}
