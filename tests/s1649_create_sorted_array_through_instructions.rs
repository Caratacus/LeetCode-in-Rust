// Tests for Problem 1649: Create Sorted Array through Instructions
// Java reference: src/test/java/g1601_1700/s1649_create_sorted_array_through_instructions/SolutionTest.java

use leetcode_in_rust::s1649::create_sorted_array_through_instructions::Solution;

#[test]
fn test_create_sorted_array() {
    assert_eq!(Solution::create_sorted_array(vec![1, 5, 6, 2]), 1);
}

#[test]
fn test_create_sorted_array2() {
    assert_eq!(Solution::create_sorted_array(vec![1, 2, 3, 6, 5, 4]), 3);
}

#[test]
fn test_create_sorted_array3() {
    assert_eq!(Solution::create_sorted_array(vec![1, 3, 3, 3, 2, 4, 2, 1, 2]), 4);
}
