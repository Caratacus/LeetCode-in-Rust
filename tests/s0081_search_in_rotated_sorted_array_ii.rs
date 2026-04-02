// Tests for Problem 0081: Search in Rotated Sorted Array II
// Java reference: src/test/java/g0001_0100/s0081_search_in_rotated_sorted_array_ii/SolutionTest.java

use leetcode_in_rust::s0081::search_in_rotated_sorted_array_ii::Solution;

#[test]
fn test_search() {
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
}

#[test]
fn test_search2() {
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}
