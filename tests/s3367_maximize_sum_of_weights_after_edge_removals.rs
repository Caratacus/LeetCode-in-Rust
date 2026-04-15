// Tests for Problem 3367: Maximize Sum of Weights After Edge Removals
// Java reference: src/test/java/g3301_3400/s3367_maximize_sum_of_weights_after_edge_removals/SolutionTest.java

use leetcode_in_rust::s3367::maximize_sum_of_weights_after_edge_removals::Solution;

#[test]
fn test_maximize_sum_of_weights() {
    assert_eq!(
        Solution::maximize_sum_of_weights(
            vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 12], vec![2, 4, 6]],
            2
        ),
        22
    );
}

#[test]
fn test_maximize_sum_of_weights2() {
    assert_eq!(
        Solution::maximize_sum_of_weights(
            vec![
                vec![0, 1, 5],
                vec![1, 2, 10],
                vec![0, 3, 15],
                vec![3, 4, 20],
                vec![3, 5, 5],
                vec![0, 6, 10]
            ],
            3
        ),
        65
    );
}

#[test]
fn test_maximize_sum_of_weights3() {
    assert_eq!(
        Solution::maximize_sum_of_weights(vec![vec![0, 1, 34], vec![0, 2, 17]], 1),
        34
    );
}
