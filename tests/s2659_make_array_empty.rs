// Tests for Problem 2659: Make Array Empty
// Java reference: src/test/java/g2601_2700/s2659_make_array_empty/SolutionTest.java

use leetcode_in_rust::s2659::make_array_empty::Solution;

#[test]
fn test_count_operations_to_empty_array() {
    assert_eq!(Solution::count_operations_to_empty_array(vec![3, 4, -1]), 5);
}

#[test]
fn test_count_operations_to_empty_array2() {
    assert_eq!(Solution::count_operations_to_empty_array(vec![1, 2, 3]), 3);
}
