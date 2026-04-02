// Tests for Problem 0396: Rotate Function
// Java reference: src/test/java/g0301_0400/s0396_rotate_function/SolutionTest.java

use leetcode_in_rust::s0396::rotate_function::Solution;

#[test]
fn test_max_rotate_function() {
    assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
}

#[test]
fn test_max_rotate_function2() {
    assert_eq!(Solution::max_rotate_function(vec![100]), 0);
}
