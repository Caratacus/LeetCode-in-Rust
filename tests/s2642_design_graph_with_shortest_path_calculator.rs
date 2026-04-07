// Tests for Problem 2642: Design Graph With Shortest Path Calculator
// Java reference: src/test/java/g2601_2700/s2642_design_graph_with_shortest_path_calculator/GraphTest.java

use leetcode_in_rust::s2642::design_graph_with_shortest_path_calculator::Graph;

#[test]
fn test_graph() {
    let mut g = Graph::new(4, vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]]);
    assert_eq!(g.shortest_path(3, 2), 6);
    assert_eq!(g.shortest_path(0, 3), -1);
    g.add_edge(vec![1, 3, 4]);
    assert_eq!(g.shortest_path(0, 3), 6);
}
