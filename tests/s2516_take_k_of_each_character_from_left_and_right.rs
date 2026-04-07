// Tests for Problem 2516: Take K of Each Character From Left and Right
// Java reference: src/test/java/g2401_2500/s2516_take_k_of_each_character_from_left_and_right/SolutionTest.java

use leetcode_in_rust::s2516::take_k_of_each_character_from_left_and_right::Solution;

#[test]
fn test_take_characters() {
    assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
}

#[test]
fn test_take_characters2() {
    assert_eq!(Solution::take_characters("a".to_string(), 1), -1);
}
