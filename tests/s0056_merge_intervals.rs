// Tests for Problem 0056: Merge Intervals
// Java reference: src/test/java/g0001_0100/s0056_merge_intervals/SolutionTest.java

use leetcode_in_rust::s0056::merge_intervals::Solution;

#[test]
fn test_merge() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let result = Solution::merge(intervals);
    assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
}

#[test]
fn test_merge2() {
    let intervals = vec![vec![1, 4], vec![4, 5]];
    let result = Solution::merge(intervals);
    assert_eq!(result, vec![vec![1, 5]]);
}
