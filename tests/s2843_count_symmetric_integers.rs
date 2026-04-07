// Tests for Problem 2843: Count Symmetric Integers
// Java reference: src/test/java/g2801_2900/s2843_count_symmetric_integers/SolutionTest.java

use leetcode_in_rust::s2843::count_symmetric_integers::Solution;

#[test]
fn test_count_symmetric_integers() {
    assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
}

#[test]
fn test_count_symmetric_integers2() {
    assert_eq!(Solution::count_symmetric_integers(1200, 1230), 4);
}
