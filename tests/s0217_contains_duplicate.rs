// Tests for Problem 0217: Contains Duplicate
// Java reference: src/test/java/g0201_0300/s0217_contains_duplicate/SolutionTest.java

use leetcode_in_rust::s0217::contains_duplicate::Solution;

#[test]
fn test_contains_duplicate() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
}

#[test]
fn test_contains_duplicate2() {
    assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
}

#[test]
fn test_contains_duplicate3() {
    assert_eq!(Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}
