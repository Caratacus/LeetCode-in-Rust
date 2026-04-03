// Tests for Problem 0882: Reachable Nodes In Subdivided Graph
// Java reference: src/test/java/g0801_0900/s0882_reachable_nodes_in_subdivided_graph/SolutionTest.java

use leetcode_in_rust::s0882::reachable_nodes_in_subdivided_graph::Solution;

#[test]
fn test_reachable_nodes() {
    assert_eq!(
        Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
        13
    );
}

#[test]
fn test_reachable_nodes2() {
    assert_eq!(
        Solution::reachable_nodes(
            vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
            10,
            4
        ),
        23
    );
}

#[test]
fn test_reachable_nodes3() {
    assert_eq!(
        Solution::reachable_nodes(
            vec![vec![1, 2, 4], vec![1, 4, 5], vec![1, 3, 1], vec![2, 3, 4], vec![3, 4, 5]],
            17,
            5
        ),
        1
    );
}
