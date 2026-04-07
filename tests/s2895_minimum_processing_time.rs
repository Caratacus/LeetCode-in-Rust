// Tests for Problem 2895: Minimum Processing Time
// Java reference: src/test/java/g2801_2900/s2895_minimum_processing_time/SolutionTest.java

use leetcode_in_rust::s2895::minimum_processing_time::Solution;

#[test]
fn test_min_processing_time() {
    assert_eq!(
        Solution::min_processing_time(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]),
        16
    );
}

#[test]
fn test_min_processing_time2() {
    assert_eq!(
        Solution::min_processing_time(vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]),
        23
    );
}
