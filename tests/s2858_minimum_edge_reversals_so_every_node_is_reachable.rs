// Tests for Problem 2858: Minimum Edge Reversals So Every Node Is Reachable
// Java reference: src/test/java/g2801_2900/s2858_minimum_edge_reversals_so_every_node_is_reachable/SolutionTest.java

use leetcode_in_rust::s2858::minimum_edge_reversals_so_every_node_is_reachable::Solution;

#[test]
fn test_min_edge_reversals() {
    assert_eq!(
        Solution::min_edge_reversals(4, vec![vec![2, 0], vec![2, 1], vec![1, 3]]),
        vec![1, 1, 0, 2]
    );
}

#[test]
fn test_min_edge_reversals2() {
    assert_eq!(
        Solution::min_edge_reversals(3, vec![vec![1, 2], vec![2, 0]]),
        vec![2, 0, 1]
    );
}
