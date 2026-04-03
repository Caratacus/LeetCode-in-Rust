// Tests for Problem 1640: Check Array Formation Through Concatenation
// Java reference: src/test/java/g1601_1700/s1640_check_array_formation_through_concatenation/SolutionTest.java

use leetcode_in_rust::s1640::check_array_formation_through_concatenation::Solution;

#[test]
fn test_can_form_array() {
    assert_eq!(Solution::can_form_array(vec![15, 88], vec![vec![88], vec![15]]), true);
}

#[test]
fn test_can_form_array2() {
    assert_eq!(Solution::can_form_array(vec![49, 18, 16], vec![vec![16, 18, 49]]), false);
}

#[test]
fn test_can_form_array3() {
    assert_eq!(Solution::can_form_array(vec![91, 4, 64, 78], vec![vec![78], vec![4, 64], vec![91]]), true);
}
