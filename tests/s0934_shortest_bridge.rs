// Tests for Problem 0934: Shortest Bridge
// Java reference: src/test/java/g0901_1000/s0934_shortest_bridge/SolutionTest.java

use leetcode_in_rust::s0934::shortest_bridge::Solution;

#[test]
fn test_shortest_bridge() {
    let result = Solution::shortest_bridge(vec![vec![0, 1], vec![1, 0]]);
    assert_eq!(result, 1);
}

#[test]
fn test_shortest_bridge2() {
    let result = Solution::shortest_bridge(vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]]);
    assert_eq!(result, 2);
}

#[test]
fn test_shortest_bridge3() {
    let result = Solution::shortest_bridge(vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ]);
    assert_eq!(result, 1);
}
