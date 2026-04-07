// Tests for Problem 2862: Maximum Element-Sum of a Complete Subset of Indices
// Java reference: src/test/java/g2801_2900/s2862_maximum_element_sum_of_a_complete_subset_of_indices/SolutionTest.java

use leetcode_in_rust::s2862::maximum_element_sum_of_a_complete_subset_of_indices::Solution;

#[test]
fn test_maximum_sum() {
    assert_eq!(Solution::maximum_sum(vec![8, 7, 3, 5, 7, 2, 4, 9]), 16);
}

#[test]
fn test_maximum_sum2() {
    assert_eq!(Solution::maximum_sum(vec![5, 10, 3, 10, 1, 13, 7, 9, 4]), 19);
}
