// Tests for Problem 1073: Adding Two Negabinary Numbers
// Java reference: src/test/java/g1001_1100/s1073_adding_two_negabinary_numbers/SolutionTest.java

use leetcode_in_rust::s1073::adding_two_negabinary_numbers::Solution;

#[test]
fn test_add_negabinary() {
    assert_eq!(
        Solution::add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1]),
        vec![1, 0, 0, 0, 0]
    );
}

#[test]
fn test_add_negabinary2() {
    assert_eq!(Solution::add_negabinary(vec![0], vec![0]), vec![0]);
}

#[test]
fn test_add_negabinary3() {
    assert_eq!(Solution::add_negabinary(vec![0], vec![1]), vec![1]);
}
