// Tests for Problem 0378: Kth Smallest Element in a Sorted Matrix
// Java reference: src/test/java/g0301_0400/s0378_kth_smallest_element_in_a_sorted_matrix/SolutionTest.java

use leetcode_in_rust::s0378::kth_smallest_element_in_a_sorted_matrix::Solution;

#[test]
fn test_kth_smallest() {
    assert_eq!(Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8), 13);
}

#[test]
fn test_kth_smallest2() {
    assert_eq!(Solution::kth_smallest(vec![vec![-5]], 1), -5);
}
