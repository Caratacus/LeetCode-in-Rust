// Tests for Problem 3515: Shortest Path in a Weighted Tree
// Java reference: src/test/java/g3501_3600/s3515_shortest_path_in_a_weighted_tree/SolutionTest.java

use leetcode_in_rust::s3515::shortest_path_in_a_weighted_tree::Solution;

#[test]
fn test_tree_queries() {
    assert_eq!(
        Solution::tree_queries(2, vec![vec![1, 2, 7]], vec![vec![2, 2], vec![1, 1, 2, 4], vec![2, 2]]),
        vec![7, 4]
    );
}

#[test]
fn test_tree_queries2() {
    assert_eq!(
        Solution::tree_queries(3, vec![vec![1, 2, 2], vec![1, 3, 4]], vec![vec![2, 1], vec![2, 3], vec![1, 1, 3, 7], vec![2, 2], vec![2, 3]]),
        vec![0, 4, 2, 7]
    );
}

#[test]
fn test_tree_queries3() {
    assert_eq!(
        Solution::tree_queries(4, vec![vec![1, 2, 2], vec![2, 3, 1], vec![3, 4, 5]], vec![vec![2, 4], vec![2, 3], vec![1, 2, 3, 3], vec![2, 2], vec![2, 3]]),
        vec![8, 3, 2, 5]
    );
}
