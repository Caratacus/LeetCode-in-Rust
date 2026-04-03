// Tests for Problem 0757: Set Intersection Size At Least Two
// Java reference: src/test/java/g0701_0800/s0757_set_intersection_size_at_least_two/SolutionTest.java

use leetcode_in_rust::s0757::set_intersection_size_at_least_two::Solution;

#[test]
fn test_intersection_size_two() {
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]),
        3
    );
}

#[test]
fn test_intersection_size_two2() {
    assert_eq!(
        Solution::intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]),
        5
    );
}
