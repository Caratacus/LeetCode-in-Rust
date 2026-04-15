// Tests for Problem 3680: Generate Schedule
// Java reference: src/test/java/g3601_3700/s3680_generate_schedule/SolutionTest.java
use leetcode_in_rust::s3680::generate_schedule::Solution;
#[test]
fn test_generate_schedule() { assert_eq!(Solution::generate_schedule(3), vec![] as Vec<Vec<i32>>); }
#[test]
fn test_generate_schedule2() {
    assert_eq!(
        Solution::generate_schedule(5),
        vec![vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 0], vec![4, 1], vec![0, 3], vec![1, 4], vec![2, 0], vec![3, 1], vec![4, 2], vec![0, 1], vec![4, 3], vec![1, 2], vec![0, 4], vec![2, 3], vec![1, 0], vec![3, 4], vec![2, 1], vec![4, 0], vec![3, 2]]
    );
}
