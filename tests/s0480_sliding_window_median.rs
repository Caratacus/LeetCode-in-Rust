// Tests for Problem 0480: Sliding Window Median
// Java reference: src/test/java/g0401_0500/s0480_sliding_window_median/SolutionTest.java

use leetcode_in_rust::s0480::sliding_window_median::Solution;

#[test]
fn test_median_sliding_window() {
    let result = Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
    assert_eq!(result, vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0]);
}

#[test]
fn test_median_sliding_window2() {
    let result = Solution::median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3);
    assert_eq!(result, vec![2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0]);
}
