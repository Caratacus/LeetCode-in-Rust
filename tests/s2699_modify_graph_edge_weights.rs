// Tests for Problem 2699: Modify Graph Edge Weights
// Java reference: src/test/java/g2601_2700/s2699_modify_graph_edge_weights/SolutionTest.java

use leetcode_in_rust::s2699::modify_graph_edge_weights::Solution;

#[test]
fn test_modified_graph_edges() {
    let result = Solution::modified_graph_edges(
        5,
        vec![vec![4, 1, -1], vec![2, 0, -1], vec![0, 3, -1], vec![4, 3, -1]],
        0,
        1,
        5,
    );
    // Expected: [[4,1,1],[2,0,1],[0,3,3],[4,3,1]]
    assert_eq!(result, vec![vec![4, 1, 1], vec![2, 0, 1], vec![0, 3, 3], vec![4, 3, 1]]);
}

#[test]
fn test_modified_graph_edges2() {
    let result = Solution::modified_graph_edges(
        3,
        vec![vec![0, 1, -1], vec![0, 2, 5]],
        0,
        2,
        6,
    );
    // Expected: empty array
    assert_eq!(result, vec![] as Vec<Vec<i32>>);
}
