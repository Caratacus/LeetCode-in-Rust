// Tests for Problem 2876: Count Visited Nodes in a Directed Graph
// Java reference: src/test/java/g2801_2900/s2876_count_visited_nodes_in_a_directed_graph/SolutionTest.java

use leetcode_in_rust::s2876::count_visited_nodes_in_a_directed_graph::Solution;

#[test]
fn test_count_visited_nodes() {
    assert_eq!(Solution::count_visited_nodes(vec![1, 2, 0, 0]), vec![3, 3, 3, 4]);
}

#[test]
fn test_count_visited_nodes2() {
    assert_eq!(Solution::count_visited_nodes(vec![1, 2, 3, 4, 0]), vec![5, 5, 5, 5, 5]);
}
