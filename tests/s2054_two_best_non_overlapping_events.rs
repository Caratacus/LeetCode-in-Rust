// Tests for Problem 2054: Two Best Non Overlapping Events
// Java reference: src/test/java/g2001_2100/s2054_two_best_non_overlapping_events/SolutionTest.java

use leetcode_in_rust::s2054::two_best_non_overlapping_events::Solution;

#[test]
fn test_max_two_events1() {
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]]),
        4
    );
}

#[test]
fn test_max_two_events2() {
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]),
        5
    );
}

#[test]
fn test_max_two_events3() {
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]]),
        8
    );
}
