// Tests for Problem 2718: Sum of Matrix After Queries
// Java reference: src/test/java/g2701_2800/s2718_sum_of_matrix_after_queries/SolutionTest.java

use leetcode_in_rust::s2718::sum_of_matrix_after_queries::Solution;

#[test]
fn test_matrix_sum_queries() {
    assert_eq!(
        Solution::matrix_sum_queries(
            3,
            vec![
                vec![0, 0, 1],
                vec![1, 2, 2],
                vec![0, 2, 3],
                vec![1, 0, 4]
            ]
        ),
        23
    );
}

#[test]
fn test_matrix_sum_queries2() {
    assert_eq!(
        Solution::matrix_sum_queries(
            3,
            vec![
                vec![0, 0, 4],
                vec![0, 1, 2],
                vec![1, 0, 1],
                vec![0, 2, 3],
                vec![1, 2, 1]
            ]
        ),
        17
    );
}
