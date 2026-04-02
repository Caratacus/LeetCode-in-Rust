// Tests for Problem 0486: Predict the Winner
// Java reference: src/test/java/g0401_0500/s0486_predict_the_winner/SolutionTest.java

use leetcode_in_rust::s0486::predict_the_winner::Solution;

#[test]
fn test_predict_the_winner() {
    assert_eq!(Solution::predict_the_winner(vec![1, 5, 2]), false);
}

#[test]
fn test_predict_the_winner2() {
    assert_eq!(Solution::predict_the_winner(vec![1, 5, 233, 7]), true);
}
