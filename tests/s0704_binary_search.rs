// Tests for Problem 0704: Binary Search
// Java reference: src/test/java/g0701_0800/s0704_binary_search/SolutionTest.java

use leetcode_in_rust::s0704::binary_search::Solution;

#[test]
fn test_search() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
}

#[test]
fn test_search2() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
