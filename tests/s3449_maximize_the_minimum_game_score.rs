// Tests for Problem 3449: Maximize the Minimum Game Score
// Java reference: src/test/java/g3401_3500/s3449_maximize_the_minimum_game_score/SolutionTest.java

use leetcode_in_rust::s3449::maximize_the_minimum_game_score::Solution;

#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![2, 4], 3), 4i64);
}

#[test]
fn test_max_score2() {
    assert_eq!(Solution::max_score(vec![1, 2, 3], 5), 2i64);
}
