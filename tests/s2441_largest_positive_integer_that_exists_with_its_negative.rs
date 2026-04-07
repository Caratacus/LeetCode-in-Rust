// Tests for Problem 2441: Largest Positive Integer That Exists With Its Negative
// Java reference: src/test/java/g2401_2500/s2441_largest_positive_integer_that_exists_with_its_negative/SolutionTest.java

use leetcode_in_rust::s2441::largest_positive_integer_that_exists_with_its_negative::Solution;

#[test]
fn test_find_max_k() {
    assert_eq!(Solution::find_max_k(vec![-1, 2, -3, 3]), 3);
}

#[test]
fn test_find_max_k2() {
    assert_eq!(Solution::find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);
}

#[test]
fn test_find_max_k3() {
    assert_eq!(Solution::find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
}
