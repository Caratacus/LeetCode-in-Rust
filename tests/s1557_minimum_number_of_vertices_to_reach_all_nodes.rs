// Tests for Problem 1557: Minimum Number of Vertices to Reach All Nodes
// Java reference: src/test/java/g1501_1600/s1557_minimum_number_of_vertices_to_reach_all_nodes/SolutionTest.java

use leetcode_in_rust::s1557::minimum_number_of_vertices_to_reach_all_nodes::Solution;

#[test]
fn test_find_smallest_set_of_vertices() {
    let mut result = Solution::find_smallest_set_of_vertices(
        6,
        vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]],
    );
    result.sort();
    assert_eq!(result, vec![0, 3]);
}

#[test]
fn test_find_smallest_set_of_vertices2() {
    let mut result = Solution::find_smallest_set_of_vertices(
        5,
        vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]],
    );
    result.sort();
    assert_eq!(result, vec![0, 2, 3]);
}
