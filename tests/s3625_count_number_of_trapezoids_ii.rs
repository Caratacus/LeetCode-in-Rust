// Tests for Problem 3625: Count Number of Trapezoids II
// Java reference: src/test/java/g3601_3700/s3625_count_number_of_trapezoids_ii/SolutionTest.java
use leetcode_in_rust::s3625::count_number_of_trapezoids_ii::Solution;
#[test]
fn test_count_trapezoids() {
    assert_eq!(Solution::count_trapezoids(vec![vec![-3, 2], vec![3, 0], vec![2, 3], vec![3, 2], vec![2, -3]]), 2);
}
#[test]
fn test_count_trapezoids2() {
    assert_eq!(Solution::count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]]), 1);
}
#[test]
fn test_count_trapezoids3() {
    assert_eq!(Solution::count_trapezoids(vec![vec![71, -89], vec![-75, -89], vec![-9, 11], vec![-24, -89], vec![-51, -89], vec![-77, -89], vec![42, 11]]), 10);
}
