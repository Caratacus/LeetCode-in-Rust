// Tests for Problem 1138: Alphabet Board Path
// Java reference: src/test/java/g1101_1200/s1138_alphabet_board_path/SolutionTest.java

use leetcode_in_rust::s1138::alphabet_board_path::Solution;

#[test]
fn test_alphabet_board_path() {
    assert_eq!(Solution::alphabet_board_path("leet".to_string()), "DDR!UURRR!!DDD!");
}

#[test]
fn test_alphabet_board_path2() {
    assert_eq!(Solution::alphabet_board_path("code".to_string()), "RR!DDRR!LUU!R!");
}
