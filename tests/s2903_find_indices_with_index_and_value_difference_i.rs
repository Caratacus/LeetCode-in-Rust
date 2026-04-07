// Tests for Problem 2903: Find Indices With Index and Value Difference I
// Java reference: src/test/java/g2901_3000/s2903_find_indices_with_index_and_value_difference_i/SolutionTest.java

use leetcode_in_rust::s2903::find_indices_with_index_and_value_difference_i::Solution;

#[test]
fn test_find_indices() {
    assert_eq!(Solution::find_indices(vec![5, 1, 4, 1], 2, 4), vec![0, 3]);
}

#[test]
fn test_find_indices2() {
    assert_eq!(Solution::find_indices(vec![2, 1], 0, 0), vec![0, 0]);
}

#[test]
fn test_find_indices3() {
    assert_eq!(Solution::find_indices(vec![1, 2, 3], 2, 4), vec![-1, -1]);
}
