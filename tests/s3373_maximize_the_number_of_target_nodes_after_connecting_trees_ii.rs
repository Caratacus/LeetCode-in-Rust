// Tests for Problem 3373: Maximize the Number of Target Nodes After Connecting Trees II
// Java reference: src/test/java/g3301_3400/s3373_maximize_the_number_of_target_nodes_after_connecting_trees_ii/SolutionTest.java

use leetcode_in_rust::s3373::maximize_the_number_of_target_nodes_after_connecting_trees_ii::Solution;

#[test]
fn test_max_target_nodes() {
    assert_eq!(
        Solution::max_target_nodes(
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 7], vec![1, 4], vec![4, 5], vec![4, 6]]
        ),
        vec![8, 7, 7, 8, 8]
    );
}

#[test]
fn test_max_target_nodes2() {
    assert_eq!(
        Solution::max_target_nodes(
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
            vec![vec![0, 1], vec![1, 2], vec![2, 3]]
        ),
        vec![3, 6, 6, 6, 6]
    );
}
