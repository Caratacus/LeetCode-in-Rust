// Tests for Problem 1579: Remove Max Number of Edges to Keep Graph Fully Traversable
// Java reference: src/test/java/g1501_1600/s1579_remove_max_number_of_edges_to_keep_graph_fully_traversable/SolutionTest.java

use leetcode_in_rust::s1579::remove_max_number_of_edges_to_keep_graph_fully_traversable::Solution;

#[test]
fn test_max_num_edges_to_remove() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![
                vec![3, 1, 2],
                vec![3, 2, 3],
                vec![1, 1, 3],
                vec![1, 2, 4],
                vec![1, 1, 2],
                vec![2, 3, 4]
            ]
        ),
        2
    );
}

#[test]
fn test_max_num_edges_to_remove2() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]]
        ),
        0
    );
}

#[test]
fn test_max_num_edges_to_remove3() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]]
        ),
        -1
    );
}
