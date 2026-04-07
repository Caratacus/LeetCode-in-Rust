// Tests for Problem 2163: Minimum Difference in Sums After Removal of Elements
// Java reference: src/test/java/g2101_2200/s2163_minimum_difference_in_sums_after_removal_of_elements/SolutionTest.java

use leetcode_in_rust::s2163::minimum_difference_in_sums_after_removal_of_elements::Solution;

#[test]
fn test_minimum_difference() {
    assert_eq!(Solution::minimum_difference(vec![3, 1, 2]), -1);
}

#[test]
fn test_minimum_difference2() {
    assert_eq!(Solution::minimum_difference(vec![7, 9, 5, 8, 1, 3]), 1);
}
