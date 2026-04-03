// Tests for Problem 1761: Minimum Degree of a Connected Trio in a Graph
// Java reference: src/test/java/g1701_1800/s1761_minimum_degree_of_a_connected_trio_in_a_graph/SolutionTest.java

use leetcode_in_rust::s1761::minimum_degree_of_a_connected_trio_in_a_graph::Solution;

#[test]
fn test_min_trio_degree() {
    assert_eq!(
        Solution::min_trio_degree(6, vec![vec![1, 2], vec![1, 3], vec![3, 2], vec![4, 1], vec![5, 2], vec![3, 6]]),
        3
    );
}

#[test]
fn test_min_trio_degree2() {
    assert_eq!(
        Solution::min_trio_degree(
            7,
            vec![
                vec![1, 3], vec![4, 1], vec![4, 3], vec![2, 5],
                vec![5, 6], vec![6, 7], vec![7, 5], vec![2, 6]
            ]
        ),
        0
    );
}
