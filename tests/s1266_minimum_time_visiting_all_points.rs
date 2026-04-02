// Tests for Problem 1266: Minimum Time Visiting All Points
// Java reference: src/test/java/g1201_1300/s1266_minimum_time_visiting_all_points/SolutionTest.java

use leetcode_in_rust::s1266::minimum_time_visiting_all_points::Solution;

#[test]
fn test_min_time_to_visit_all_points() {
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
        7
    );
}

#[test]
fn test_min_time_to_visit_all_points2() {
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
        5
    );
}
