// Tests for Problem 0424: Longest Repeating Character Replacement
// Java reference: src/test/java/g0401_0500/s0424_longest_repeating_character_replacement/SolutionTest.java

use leetcode_in_rust::s0424::longest_repeating_character_replacement::Solution;

#[test]
fn test_character_replacement() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
}

#[test]
fn test_character_replacement2() {
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}
