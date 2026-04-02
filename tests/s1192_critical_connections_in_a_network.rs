// Tests for Problem 1192: Critical Connections in a Network
// Java reference: src/test/java/g1101_1200/s1192_critical_connections_in_a_network/SolutionTest.java

use leetcode_in_rust::s1192::critical_connections_in_a_network::Solution;

#[test]
fn test_critical_connections() {
    let mut result = Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]);
    for conn in &mut result {
        conn.sort();
    }
    result.sort();
    assert_eq!(result, vec![vec![1, 3]]);
}

#[test]
fn test_critical_connections2() {
    let mut result = Solution::critical_connections(2, vec![vec![0, 1]]);
    for conn in &mut result {
        conn.sort();
    }
    result.sort();
    assert_eq!(result, vec![vec![0, 1]]);
}
