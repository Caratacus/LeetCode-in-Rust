// Tests for Problem 3553: Minimum Weighted Subgraph with the Required Paths II
// Java reference: src/test/java/g3501_3600/s3553_minimum_weighted_subgraph_with_the_required_paths_ii/SolutionTest.java

use leetcode_in_rust::s3553::minimum_weighted_subgraph_with_the_required_paths_ii::Solution;

#[test]
fn test_minimum_weight() {
    assert_eq!(Solution::minimum_weight(vec![vec![0, 1, 2], vec![1, 2, 3], vec![1, 3, 5], vec![1, 4, 4], vec![2, 5, 6]], vec![vec![2, 3, 4], vec![0, 2, 5]]), vec![12, 11]);
}

#[test]
fn test_minimum_weight2() {
    assert_eq!(Solution::minimum_weight(vec![vec![1, 0, 8], vec![0, 2, 7]], vec![vec![0, 1, 2]]), vec![15]);
}

#[test]
fn test_minimum_weight3() {
    assert_eq!(Solution::minimum_weight(vec![vec![1, 0, 4], vec![2, 0, 5]], vec![vec![1, 0, 2]]), vec![9]);
}
