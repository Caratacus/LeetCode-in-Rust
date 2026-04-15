// Tests for Problem 3488: Closest Equal Element Queries
// Java reference: src/test/java/g3401_3500/s3488_closest_equal_element_queries/SolutionTest.java

use leetcode_in_rust::s3488::closest_equal_element_queries::Solution;

#[test]
fn test_solve_queries() {
    assert_eq!(
        Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
        vec![2, -1, 3]
    );
}

#[test]
fn test_solve_queries2() {
    assert_eq!(
        Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
        vec![-1, -1, -1, -1]
    );
}
