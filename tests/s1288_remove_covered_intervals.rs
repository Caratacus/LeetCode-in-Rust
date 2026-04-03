// Tests for Problem 1288: Remove Covered Intervals
// Java reference: src/test/java/g1201_1300/s1288_remove_covered_intervals/SolutionTest.java

use leetcode_in_rust::s1288::remove_covered_intervals::Solution;

#[test]
fn test_remove_covered_intervals() {
    assert_eq!(
        Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
        2
    );
}

#[test]
fn test_remove_covered_intervals2() {
    assert_eq!(Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]), 1);
}
