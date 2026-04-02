// Tests for Problem 0526: Beautiful Arrangement
// Java reference: src/test/java/g0501_0600/s0526_beautiful_arrangement/SolutionTest.java

use leetcode_in_rust::s0526::beautiful_arrangement::Solution;

#[test]
fn test_count_arrangement() {
    assert_eq!(Solution::count_arrangement(2), 2);
}

#[test]
fn test_count_arrangement2() {
    assert_eq!(Solution::count_arrangement(1), 1);
}
