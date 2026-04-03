// Tests for Problem 1776: Car Fleet II
// Java reference: src/test/java/g1701_1800/s1776_car_fleet_ii/SolutionTest.java

use leetcode_in_rust::s1776::car_fleet_ii::Solution;

#[test]
fn test_get_collision_times() {
    let result = Solution::get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]]);
    let expected = vec![1.00000, -1.00000, 3.00000, -1.00000];
    for i in 0..result.len() {
        assert!((result[i] - expected[i]).abs() < 1e-5);
    }
}

#[test]
fn test_get_collision_times2() {
    let result = Solution::get_collision_times(vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]]);
    let expected = vec![2.00000, 1.00000, 1.50000, -1.00000];
    for i in 0..result.len() {
        assert!((result[i] - expected[i]).abs() < 1e-5);
    }
}
