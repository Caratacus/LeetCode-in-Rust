// Tests for Problem 1901: Find a Peak Element II
// Java reference: src/test/java/g1901_2000/s1901_find_a_peak_element_ii/SolutionTest.java

use leetcode_in_rust::s1901::find_a_peak_element_ii::Solution;

#[test]
fn test_find_peak_grid() {
    // Note: Multiple valid answers possible
    let result = Solution::find_peak_grid(vec![vec![1, 4], vec![3, 2]]);
    assert!(result == vec![0, 1] || result == vec![1, 0] || result == vec![1, 1]);
}

#[test]
fn test_find_peak_grid2() {
    let result = Solution::find_peak_grid(vec![vec![10, 20, 15], vec![21, 30, 14], vec![7, 16, 32]]);
    assert!(result == vec![1, 1] || result == vec![0, 1] || result == vec![2, 2]);
}
