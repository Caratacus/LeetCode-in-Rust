// Tests for Problem 1306: Jump Game III
// Java reference: src/test/java/g1301_1400/s1306_jump_game_iii/SolutionTest.java

use leetcode_in_rust::s1306::jump_game_iii::Solution;

#[test]
fn test_can_reach() {
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
}

#[test]
fn test_can_reach2() {
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
}

#[test]
fn test_can_reach3() {
    assert_eq!(Solution::can_reach(vec![3, 0, 2, 1, 2], 2), false);
}
