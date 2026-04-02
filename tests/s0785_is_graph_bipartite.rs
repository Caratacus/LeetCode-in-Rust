// Tests for Problem 0785: Is Graph Bipartite
// Java reference: src/test/java/g0701_0800/s0785_is_graph_bipartite/SolutionTest.java

use leetcode_in_rust::s0785::is_graph_bipartite::Solution;

#[test]
fn test_is_bipartite() {
    assert_eq!(
        Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
        false
    );
}

#[test]
fn test_is_bipartite2() {
    assert_eq!(
        Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
        true
    );
}
