// Tests for Problem 0514: Freedom Trail
// Java reference: src/test/java/g0501_0600/s0514_freedom_trail/SolutionTest.java

use leetcode_in_rust::s0514::freedom_trail::Solution;

#[test]
fn test_find_rotate_steps() {
    assert_eq!(Solution::find_rotate_steps("godding".to_string(), "gd".to_string()), 4);
}

#[test]
fn test_find_rotate_steps2() {
    assert_eq!(Solution::find_rotate_steps("godding".to_string(), "godding".to_string()), 13);
}
