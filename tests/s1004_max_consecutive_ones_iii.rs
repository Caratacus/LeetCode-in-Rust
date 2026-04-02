// Tests for Problem 1004: Max Consecutive Ones III
// Java reference: src/test/java/g1001_1100/s1004_max_consecutive_ones_iii/SolutionTest.java

use leetcode_in_rust::s1004::max_consecutive_ones_iii::Solution;

#[test]
fn test_longest_ones() {
    assert_eq!(Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
}

#[test]
fn test_longest_ones2() {
    assert_eq!(
        Solution::longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        ),
        10
    );
}
