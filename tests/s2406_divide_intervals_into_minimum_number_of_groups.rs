// Tests for Problem 2406: Divide Intervals Into Minimum Number of Groups
// Java reference: src/test/java/g2401_2500/s2406_divide_intervals_into_minimum_number_of_groups/SolutionTest.java

use leetcode_in_rust::s2406::divide_intervals_into_minimum_number_of_groups::Solution;

#[test]
fn test_min_groups() {
    assert_eq!(
        Solution::min_groups(vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]]),
        3
    );
}

#[test]
fn test_min_groups2() {
    assert_eq!(
        Solution::min_groups(vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]]),
        1
    );
}
