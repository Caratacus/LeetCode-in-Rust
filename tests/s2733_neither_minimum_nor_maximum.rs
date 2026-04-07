// Tests for Problem 2733: Neither Minimum nor Maximum
// Java reference: src/test/java/g2701_2800/s2733_neither_minimum_nor_maximum/SolutionTest.java

use leetcode_in_rust::s2733::neither_minimum_nor_maximum::Solution;

#[test]
fn test_find_non_min_or_max() {
    assert_eq!(Solution::find_non_min_or_max(vec![3, 2, 1, 4]), 3);
}

#[test]
fn test_find_non_min_or_max2() {
    assert_eq!(Solution::find_non_min_or_max(vec![1, 2]), -1);
}

#[test]
fn test_find_non_min_or_max3() {
    assert_eq!(Solution::find_non_min_or_max(vec![2, 1, 3]), 2);
}
