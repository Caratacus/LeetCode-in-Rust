// Tests for Problem 0834: Sum of Distances in Tree
// Java reference: src/test/java/g0801_0900/s0834_sum_of_distances_in_tree/SolutionTest.java

use leetcode_in_rust::s0834::sum_of_distances_in_tree::Solution;

#[test]
fn test_sum_of_distances_in_tree() {
    assert_eq!(
        Solution::sum_of_distances_in_tree(
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
        ),
        vec![8, 12, 6, 10, 10, 10]
    );
}

#[test]
fn test_sum_of_distances_in_tree2() {
    assert_eq!(
        Solution::sum_of_distances_in_tree(1, vec![]),
        vec![0]
    );
}

#[test]
fn test_sum_of_distances_in_tree3() {
    assert_eq!(
        Solution::sum_of_distances_in_tree(
            2,
            vec![vec![1, 0]]
        ),
        vec![1, 1]
    );
}
