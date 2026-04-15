// Tests for Problem 3607: Power Grid Maintenance
// Java reference: src/test/java/g3601_3700/s3607_power_grid_maintenance/SolutionTest.java
use leetcode_in_rust::s3607::power_grid_maintenance::Solution;
#[test]
fn test_process_queries() {
    assert_eq!(
        Solution::process_queries(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
            vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
        ),
        vec![3, 2, 3]
    );
}
#[test]
fn test_process_queries2() {
    assert_eq!(
        Solution::process_queries(3, vec![], vec![vec![1, 1], vec![2, 1], vec![1, 1]]),
        vec![1, -1]
    );
}
