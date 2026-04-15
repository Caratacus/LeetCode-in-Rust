// Tests for Problem 3419: Minimize the Maximum Edge Weight of Graph
// Java reference: src/test/java/g3401_3500/s3419_minimize_the_maximum_edge_weight_of_graph/SolutionTest.java

use leetcode_in_rust::s3419::minimize_the_maximum_edge_weight_of_graph::Solution;

#[test]
fn test_min_max_weight() {
    assert_eq!(
        Solution::min_max_weight(
            5,
            vec![vec![1, 0, 1], vec![2, 0, 2], vec![3, 0, 1], vec![4, 3, 1], vec![2, 1, 1]],
            2
        ),
        1
    );
}

#[test]
fn test_min_max_weight2() {
    assert_eq!(
        Solution::min_max_weight(
            5,
            vec![vec![0, 1, 1], vec![0, 2, 2], vec![0, 3, 1], vec![0, 4, 1], vec![1, 2, 1], vec![1, 4, 1]],
            1
        ),
        -1
    );
}

#[test]
fn test_min_max_weight3() {
    assert_eq!(
        Solution::min_max_weight(
            5,
            vec![vec![1, 2, 1], vec![1, 3, 3], vec![1, 4, 5], vec![2, 3, 2], vec![3, 4, 2], vec![4, 0, 1]],
            1
        ),
        2
    );
}

#[test]
fn test_min_max_weight4() {
    assert_eq!(
        Solution::min_max_weight(
            5,
            vec![vec![1, 2, 1], vec![1, 3, 3], vec![1, 4, 5], vec![2, 3, 2], vec![4, 0, 1]],
            1
        ),
        -1
    );
}
