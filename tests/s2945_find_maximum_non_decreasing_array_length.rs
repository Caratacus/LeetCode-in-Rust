// Tests for Problem 2945: Find Maximum Non-decreasing Array Length
// Java reference: src/test/java/g2901_3000/s2945_find_maximum_non_decreasing_array_length/SolutionTest.java

use leetcode_in_rust::s2945::find_maximum_non_decreasing_array_length::Solution;

#[test]
fn test_find_maximum_length() {
    assert_eq!(Solution::find_maximum_length(vec![5, 2, 2]), 1);
}

#[test]
fn test_find_maximum_length2() {
    assert_eq!(Solution::find_maximum_length(vec![1, 2, 3, 4]), 4);
}

#[test]
fn test_find_maximum_length3() {
    assert_eq!(Solution::find_maximum_length(vec![4, 3, 2, 6]), 3);
}
