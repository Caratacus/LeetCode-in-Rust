// Tests for Problem 3275: K-th Nearest Obstacle Queries
// Java reference: src/test/java/g3201_3300/s3275_k_th_nearest_obstacle_queries/SolutionTest.java

use leetcode_in_rust::s3275::k_th_nearest_obstacle_queries::Solution;

#[test]
fn test_results_array() {
    assert_eq!(
        Solution::results_array(vec![vec![1, 2], vec![3, 4], vec![2, 3], vec![-3, 0]], 2),
        vec![-1, 7, 5, 3]
    );
}

#[test]
fn test_results_array2() {
    assert_eq!(
        Solution::results_array(vec![vec![5, 5], vec![4, 4], vec![3, 3]], 1),
        vec![10, 8, 6]
    );
}
