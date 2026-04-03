// Tests for Problem 0717: 1-bit and 2-bit Characters
// Java reference: src/test/java/g0701_0800/s0717_1_bit_and_2_bit_characters/SolutionTest.java

use leetcode_in_rust::s0717::p1_bit_and_2_bit_characters::Solution;

#[test]
fn test_is_one_bit_character() {
    assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
}

#[test]
fn test_is_one_bit_character2() {
    assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
}
