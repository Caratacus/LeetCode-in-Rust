// Tests for Problem 1550: Three Consecutive Odds
// Java reference: src/test/java/g1501_1600/s1550_three_consecutive_odds/SolutionTest.java

use leetcode_in_rust::s1550::three_consecutive_odds::Solution;

#[test]
fn test_three_consecutive_odds() {
    assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
}

#[test]
fn test_three_consecutive_odds2() {
    assert_eq!(
        Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
        true
    );
}
