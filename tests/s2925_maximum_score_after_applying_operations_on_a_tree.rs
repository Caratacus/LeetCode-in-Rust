// Tests for Problem 2925: Maximum Score After Applying Operations on a Tree
// Java reference: src/test/java/g2901_3000/s2925_maximum_score_after_applying_operations_on_a_tree/SolutionTest.java

use leetcode_in_rust::s2925::maximum_score_after_applying_operations_on_a_tree::Solution;

#[test]
fn test_maximum_score_after_operations() {
    assert_eq!(
        Solution::maximum_score_after_operations(
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![4, 5]],
            vec![5, 2, 5, 2, 1, 1]
        ),
        11
    );
}

#[test]
fn test_maximum_score_after_operations2() {
    assert_eq!(
        Solution::maximum_score_after_operations(
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]],
            vec![20, 10, 9, 7, 4, 3, 5]
        ),
        40
    );
}
