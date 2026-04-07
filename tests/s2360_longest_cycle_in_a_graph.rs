// Tests for Problem 2360: Longest Cycle in a Graph
// Java reference: src/test/java/g2301_2400/s2360_longest_cycle_in_a_graph/SolutionTest.java

use leetcode_in_rust::s2360::longest_cycle_in_a_graph::Solution;

#[test]
fn test_longest_cycle() {
    assert_eq!(Solution::longest_cycle(vec![3, 3, 4, 2, 3]), 3);
}

#[test]
fn test_longest_cycle2() {
    assert_eq!(Solution::longest_cycle(vec![2, -1, 3, 1]), -1);
}
