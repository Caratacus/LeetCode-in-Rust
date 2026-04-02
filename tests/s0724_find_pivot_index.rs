// Tests for Problem 0724: Find Pivot Index
// Java reference: src/test/java/g0701_0800/s0724_find_pivot_index/SolutionTest.java

use leetcode_in_rust::s0724::find_pivot_index::Solution;

#[test]
fn test_pivot_index() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
}

#[test]
fn test_pivot_index2() {
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
}

#[test]
fn test_pivot_index3() {
    assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
}
