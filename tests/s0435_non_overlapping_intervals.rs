// Tests for Problem 0435: Non Overlapping Intervals
// Java reference: src/test/java/g0401_0500/s0435_non_overlapping_intervals/SolutionTest.java

use leetcode_in_rust::s0435::non_overlapping_intervals::Solution;

#[test]
fn test_erase_overlap_intervals() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
}

#[test]
fn test_erase_overlap_intervals2() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );
}

#[test]
fn test_erase_overlap_intervals3() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
        0
    );
}
