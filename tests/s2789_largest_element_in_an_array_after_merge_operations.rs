// Tests for Problem 2789: Largest Element in an Array After Merge Operations
// Java reference: src/test/java/g2701_2800/s2789_largest_element_in_an_array_after_merge_operations/SolutionTest.java

use leetcode_in_rust::s2789::largest_element_in_an_array_after_merge_operations::Solution;

#[test]
fn test_max_array_value() {
    assert_eq!(Solution::max_array_value(vec![2, 3, 7, 9, 3]), 21);
}

#[test]
fn test_max_array_value2() {
    assert_eq!(Solution::max_array_value(vec![5, 3, 3]), 11);
}
