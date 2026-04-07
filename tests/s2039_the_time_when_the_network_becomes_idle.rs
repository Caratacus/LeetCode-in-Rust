// Tests for Problem 2039: The Time When the Network Becomes Idle
// Java reference: src/test/java/g2001_2100/s2039_the_time_when_the_network_becomes_idle/SolutionTest.java

use leetcode_in_rust::s2039::the_time_when_the_network_becomes_idle::Solution;

#[test]
fn test_network_becomes_idle() {
    assert_eq!(
        Solution::network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
        8
    );
}

#[test]
fn test_network_becomes_idle2() {
    assert_eq!(
        Solution::network_becomes_idle(
            vec![vec![0, 1], vec![0, 2], vec![1, 2]],
            vec![0, 10, 10]
        ),
        3
    );
}
