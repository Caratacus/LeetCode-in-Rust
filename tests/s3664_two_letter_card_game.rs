// Tests for Problem 3664: Two Letter Card Game
// Java reference: src/test/java/g3601_3700/s3664_two_letter_card_game/SolutionTest.java
use leetcode_in_rust::s3664::two_letter_card_game::Solution;
#[test]
fn test_score() { assert_eq!(Solution::score(vec!["aa".to_string(), "ab".to_string(), "ba".to_string(), "ac".to_string()], 'a'), 2); }
#[test]
fn test_score2() { assert_eq!(Solution::score(vec!["aa".to_string(), "ab".to_string(), "ba".to_string()], 'a'), 1); }
#[test]
fn test_score3() { assert_eq!(Solution::score(vec!["aa".to_string(), "ab".to_string(), "ba".to_string(), "ac".to_string()], 'b'), 0); }
