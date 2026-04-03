// Tests for Problem 0743: Network Delay Time
// Java reference: src/test/java/g0701_0800/s0743_network_delay_time/SolutionTest.java

use leetcode_in_rust::s0743::network_delay_time::Solution;

#[test]
fn test_network_delay_time() {
    assert_eq!(
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
}

#[test]
fn test_network_delay_time2() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
}

#[test]
fn test_network_delay_time3() {
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}
