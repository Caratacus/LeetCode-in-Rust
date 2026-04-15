// Tests for Problem 3623: Count Number of Trapezoids I
// Java reference: src/test/java/g3601_3700/s3623_count_number_of_trapezoids_i/SolutionTest.java
use leetcode_in_rust::s3623::count_number_of_trapezoids_i::Solution;
#[test]
fn test_count_trapezoids() {
    assert_eq!(Solution::count_trapezoids(vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]]), 3);
}
#[test]
fn test_count_trapezoids2() {
    assert_eq!(Solution::count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]]), 1);
}
