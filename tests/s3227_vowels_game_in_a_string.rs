// Tests for Problem 3227: Vowels Game in a String
// Java reference: src/test/java/g3201_3300/s3227_vowels_game_in_a_string/SolutionTest.java

use leetcode_in_rust::s3227::vowels_game_in_a_string::Solution;

#[test]
fn test_does_alice_win() {
    assert_eq!(Solution::does_alice_win("leetcoder".to_string()), true);
}

#[test]
fn test_does_alice_win2() {
    assert_eq!(Solution::does_alice_win("bbcd".to_string()), false);
}
