// Tests for Problem 1051: Height Checker
// Java reference: src/test/java/g1001_1100/s1051_height_checker/SolutionTest.java

use leetcode_in_rust::s1051::height_checker::Solution;

#[test]
fn test_height_checker() {
    assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
}

#[test]
fn test_height_checker2() {
    assert_eq!(Solution::height_checker(vec![5, 1, 2, 3, 4]), 5);
}

#[test]
fn test_height_checker3() {
    assert_eq!(Solution::height_checker(vec![1, 2, 3, 4, 5]), 0);
}
