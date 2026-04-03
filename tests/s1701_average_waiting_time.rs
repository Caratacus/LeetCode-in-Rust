// Tests for Problem 1701: Average Waiting Time
// Java reference: src/test/java/g1701_1800/s1701_average_waiting_time/SolutionTest.java

use leetcode_in_rust::s1701::average_waiting_time::Solution;

#[test]
fn test_average_waiting_time() {
    assert_eq!(Solution::average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]), 5.0);
}

#[test]
fn test_average_waiting_time2() {
    assert_eq!(Solution::average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]), 3.25);
}
