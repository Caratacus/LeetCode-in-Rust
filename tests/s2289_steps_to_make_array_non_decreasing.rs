// Tests for Problem 2289: Steps to Make Array Non-decreasing
// Java reference: src/test/java/g2201_2300/s2289_steps_to_make_array_non_decreasing/SolutionTest.java

use leetcode_in_rust::s2289::steps_to_make_array_non_decreasing::Solution;

#[test]
fn test_total_steps() {
    assert_eq!(
        Solution::total_steps(vec![5, 3, 4, 4, 7, 3, 6, 11, 8, 5, 11]),
        3
    );
}

#[test]
fn test_total_steps2() {
    assert_eq!(Solution::total_steps(vec![4, 5, 7, 7, 13]), 0);
}
