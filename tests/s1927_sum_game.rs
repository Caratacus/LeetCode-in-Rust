// Tests for Problem 1927: Sum Game
// Java reference: src/test/java/g1901_2000/s1927_sum_game/SolutionTest.java

use leetcode_in_rust::s1927::sum_game::Solution;

#[test]
fn test_sum_game() {
    assert_eq!(Solution::sum_game("5023".to_string()), false);
}

#[test]
fn test_sum_game2() {
    assert_eq!(Solution::sum_game("25??".to_string()), true);
}

#[test]
fn test_sum_game3() {
    assert_eq!(Solution::sum_game("?3295???".to_string()), false);
}
