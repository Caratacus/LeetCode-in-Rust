// Tests for Problem 1765: Map of Highest Peak
// Java reference: src/test/java/g1701_1800/s1765_map_of_highest_peak/SolutionTest.java

use leetcode_in_rust::s1765::map_of_highest_peak::Solution;

#[test]
fn test_highest_peak() {
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]),
        vec![vec![1, 0], vec![2, 1]]
    );
}

#[test]
fn test_highest_peak2() {
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
        vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
    );
}
