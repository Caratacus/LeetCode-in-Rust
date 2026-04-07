// Tests for Problem 2846: Minimum Edge Weight Equilibrium Queries in a Tree
// Java reference: src/test/java/g2801_2900/s2846_minimum_edge_weight_equilibrium_queries_in_a_tree/SolutionTest.java

use leetcode_in_rust::s2846::minimum_edge_weight_equilibrium_queries_in_a_tree::Solution;

#[test]
fn test_min_operations_queries() {
    assert_eq!(
        Solution::min_operations_queries(
            7,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![3, 4, 2], vec![4, 5, 2], vec![5, 6, 2]],
            vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]]
        ),
        vec![0, 0, 1, 3]
    );
}

#[test]
fn test_min_operations_queries2() {
    assert_eq!(
        Solution::min_operations_queries(
            8,
            vec![vec![1, 2, 6], vec![1, 3, 4], vec![2, 4, 6], vec![2, 5, 3], vec![3, 6, 6], vec![3, 0, 8], vec![7, 0, 2]],
            vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]]
        ),
        vec![1, 2, 2, 3]
    );
}
