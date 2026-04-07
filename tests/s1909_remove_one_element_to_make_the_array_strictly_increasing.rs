// Tests for Problem 1909: Remove One Element to Make the Array Strictly Increasing
// Java reference: src/test/java/g1901_2000/s1909_remove_one_element_to_make_the_array_strictly_increasing/SolutionTest.java

use leetcode_in_rust::s1909::remove_one_element_to_make_the_array_strictly_increasing::Solution;

#[test]
fn test_can_be_increasing() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
}

#[test]
fn test_can_be_increasing2() {
    assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
}

#[test]
fn test_can_be_increasing3() {
    assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
}
