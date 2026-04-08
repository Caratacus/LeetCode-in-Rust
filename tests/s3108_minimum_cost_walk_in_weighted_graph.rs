// Tests for Problem 3108: Minimum Cost Walk in Weighted Graph
// Java reference: src/test/java/g3101_3200/s3108_minimum_cost_walk_in_weighted_graph/SolutionTest.java

use leetcode_in_rust::s3108::minimum_cost_walk_in_weighted_graph::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(
        Solution::minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]]
        ),
        vec![1, -1]
    );
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(
        Solution::minimum_cost(
            3,
            vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
            vec![vec![1, 2]]
        ),
        vec![0]
    );
}
