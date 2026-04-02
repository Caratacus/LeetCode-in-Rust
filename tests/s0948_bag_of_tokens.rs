// Tests for Problem 0948: Bag of Tokens
// Java reference: src/test/java/g0901_1000/s0948_bag_of_tokens/SolutionTest.java

use leetcode_in_rust::s0948::bag_of_tokens::Solution;

#[test]
fn test_bag_of_tokens_score() {
    assert_eq!(Solution::bag_of_tokens_score(vec![100, 200], 150), 1);
}

#[test]
fn test_bag_of_tokens_score2() {
    assert_eq!(Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
}
