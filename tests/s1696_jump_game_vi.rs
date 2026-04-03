// Tests for Problem 1696: Jump Game VI
// Java reference: src/test/java/g1601_1700/s1696_jump_game_vi/SolutionTest.java

use leetcode_in_rust::s1696::jump_game_vi::Solution;

#[test]
fn test_max_result() {
    assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
}

#[test]
fn test_max_result2() {
    assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
}

#[test]
fn test_max_result3() {
    assert_eq!(Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2), 0);
}
