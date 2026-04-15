// Tests for Problem 3608: Minimum Time for K Connected Components
// Java reference: src/test/java/g3601_3700/s3608_minimum_time_for_k_connected_components/SolutionTest.java
use leetcode_in_rust::s3608::minimum_time_for_k_connected_components::Solution;
#[test]
fn test_min_time() {
    assert_eq!(Solution::min_time(2, vec![vec![0, 1, 3]], 2), 3);
}
#[test]
fn test_min_time2() {
    assert_eq!(Solution::min_time(3, vec![vec![0, 1, 2], vec![1, 2, 4]], 3), 4);
}
#[test]
fn test_min_time3() {
    assert_eq!(Solution::min_time(3, vec![vec![0, 2, 5]], 2), 0);
}
#[test]
fn test_min_time4() {
    assert_eq!(Solution::min_time(3, vec![vec![2, 1, 1469], vec![1, 0, 5701]], 2), 1469);
}
