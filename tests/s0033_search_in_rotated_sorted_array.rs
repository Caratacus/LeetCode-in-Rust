// Tests for Problem 0033: Search in Rotated Sorted Array
// Java reference: src/test/java/g0001_0100/s0033_search_in_rotated_sorted_array/SolutionTest.java

use leetcode_in_rust::s0033::search_in_rotated_sorted_array::Solution;

#[test]
fn test_search() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
}

#[test]
fn test_search2() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}

#[test]
fn test_search3() {
    assert_eq!(Solution::search(vec![1], 0), -1);
}

#[test]
fn test_search4() {
    assert_eq!(Solution::search(vec![1, 2, 3, 4, 5, 6], 4), 3);
}
