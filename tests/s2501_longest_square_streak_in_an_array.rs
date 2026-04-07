// Tests for Problem 2501: Longest Square Streak in an Array
// Java reference: src/test/java/g2401_2500/s2501_longest_square_streak_in_an_array/SolutionTest.java

use leetcode_in_rust::s2501::longest_square_streak_in_an_array::Solution;

#[test]
fn test_longest_square_streak() {
    assert_eq!(Solution::longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
}

#[test]
fn test_longest_square_streak2() {
    assert_eq!(Solution::longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
}
