// Tests for Problem 0239: Sliding Window Maximum
// Java reference: src/test/java/g0201_0300/s0239_sliding_window_maximum/SolutionTest.java

use leetcode_in_rust::s0239::sliding_window_maximum::Solution;

#[test]
fn test_max_sliding_window() {
    assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
}

#[test]
fn test_max_sliding_window2() {
    assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
}
