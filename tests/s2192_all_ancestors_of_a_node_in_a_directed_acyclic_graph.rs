// Tests for Problem 2192: All Ancestors of a Node in a Directed Acyclic Graph
// Java reference: src/test/java/g2101_2200/s2192_all_ancestors_of_a_node_in_a_directed_acyclic_graph/SolutionTest.java

use leetcode_in_rust::s2192::all_ancestors_of_a_node_in_a_directed_acyclic_graph::Solution;

#[test]
fn test_get_ancestors() {
    let edges = vec![
        vec![0, 3],
        vec![0, 4],
        vec![1, 3],
        vec![2, 4],
        vec![2, 7],
        vec![3, 5],
        vec![3, 6],
        vec![3, 7],
        vec![4, 6],
    ];
    let result = Solution::get_ancestors(8, edges);
    assert_eq!(result[0], vec![]);
    assert_eq!(result[1], vec![]);
    assert_eq!(result[2], vec![]);
    assert_eq!(result[3], vec![0, 1]);
    assert_eq!(result[4], vec![0, 2]);
    assert_eq!(result[5], vec![0, 1, 3]);
    assert_eq!(result[6], vec![0, 1, 2, 3, 4]);
    assert_eq!(result[7], vec![0, 1, 2, 3]);
}

#[test]
fn test_get_ancestors2() {
    let edges = vec![
        vec![0, 1],
        vec![0, 2],
        vec![0, 3],
        vec![0, 4],
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ];
    let result = Solution::get_ancestors(8, edges);
    assert_eq!(result[0], vec![]);
    assert_eq!(result[1], vec![0]);
    assert_eq!(result[2], vec![0, 1]);
    assert_eq!(result[3], vec![0, 1, 2]);
    assert_eq!(result[4], vec![0, 1, 2, 3]);
    assert_eq!(result[5], vec![]);
    assert_eq!(result[6], vec![]);
    assert_eq!(result[7], vec![]);
}
