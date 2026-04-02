// Tests for Problem 0986: Interval List Intersections
// Java reference: src/test/java/g0901_1000/s0986_interval_list_intersections/SolutionTest.java

use leetcode_in_rust::s0986::interval_list_intersections::Solution;

#[test]
fn test_interval_intersection() {
    assert_eq!(
        Solution::interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        ),
        vec![vec![1, 2], vec![5, 5], vec![8, 10], vec![15, 23], vec![24, 24], vec![25, 25]]
    );
}

#[test]
fn test_interval_intersection2() {
    assert_eq!(
        Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]),
        vec![] as Vec<Vec<i32>>
    );
}
