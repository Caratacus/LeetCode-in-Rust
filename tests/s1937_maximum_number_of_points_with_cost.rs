// Tests for Problem 1937: Maximum Number of Points with Cost
// Java reference: src/test/java/g1901_2000/s1937_maximum_number_of_points_with_cost/SolutionTest.java

use leetcode_in_rust::s1937::maximum_number_of_points_with_cost::Solution;

#[test]
fn test_max_points() {
    assert_eq!(
        Solution::max_points(vec![vec![1, 2, 3], vec![1, 5, 1], vec![3, 1, 1]]),
        9
    );
}

#[test]
fn test_max_points2() {
    assert_eq!(
        Solution::max_points(vec![vec![1, 5], vec![2, 3], vec![4, 2]]),
        11
    );
}
