// Tests for Problem 1654: Minimum Jumps to Reach Home
// Java reference: src/test/java/g1601_1700/s1654_minimum_jumps_to_reach_home/SolutionTest.java

use leetcode_in_rust::s1654::minimum_jumps_to_reach_home::Solution;

#[test]
fn test_minimum_jumps() {
    assert_eq!(Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9), 3);
}

#[test]
fn test_minimum_jumps2() {
    assert_eq!(Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11), -1);
}

#[test]
fn test_minimum_jumps3() {
    assert_eq!(Solution::minimum_jumps(vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7), 2);
}
