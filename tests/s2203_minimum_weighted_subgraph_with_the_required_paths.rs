// Tests for Problem 2203: Minimum Weighted Subgraph With the Required Paths
// Java reference: src/test/java/g2201_2300/s2203_minimum_weighted_subgraph_with_the_required_paths/SolutionTest.java

use leetcode_in_rust::s2203::minimum_weighted_subgraph_with_the_required_paths::Solution;

#[test]
fn test_minimum_weight() {
    assert_eq!(
        Solution::minimum_weight(
            6,
            vec![
                vec![0, 2, 2],
                vec![0, 5, 6],
                vec![1, 0, 3],
                vec![1, 4, 5],
                vec![2, 1, 1],
                vec![2, 3, 3],
                vec![2, 3, 4],
                vec![3, 4, 2],
                vec![4, 5, 1]
            ],
            0,
            1,
            5
        ),
        9
    );
}

#[test]
fn test_minimum_weight2() {
    assert_eq!(
        Solution::minimum_weight(3, vec![vec![0, 1, 1], vec![2, 1, 1]], 0, 1, 2),
        -1
    );
}
