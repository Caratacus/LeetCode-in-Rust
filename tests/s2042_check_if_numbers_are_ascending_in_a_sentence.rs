// Tests for Problem 2042: Check if Numbers Are Ascending in a Sentence
// Java reference: src/test/java/g2001_2100/s2042_check_if_numbers_are_ascending_in_a_sentence/SolutionTest.java

use leetcode_in_rust::s2042::check_if_numbers_are_ascending_in_a_sentence::Solution;

#[test]
fn test_are_numbers_ascending() {
    assert_eq!(
        Solution::are_numbers_ascending(
            "1 box has 3 blue 4 red 6 green and 12 yellow marbles".to_string()
        ),
        true
    );
}

#[test]
fn test_are_numbers_ascending2() {
    assert_eq!(
        Solution::are_numbers_ascending("hello world 5 x 5".to_string()),
        false
    );
}

#[test]
fn test_are_numbers_ascending3() {
    assert_eq!(
        Solution::are_numbers_ascending(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s".to_string()
        ),
        false
    );
}
