// Tests for Problem 1871: Jump Game VII
// Java reference: src/test/java/g1801_1900/s1871_jump_game_vii/SolutionTest.java

use leetcode_in_rust::s1871::jump_game_vii::Solution;

#[test]
fn test_can_reach() {
    assert_eq!(
        Solution::can_reach("011010".to_string(), 2, 3),
        true
    );
}

#[test]
fn test_can_reach2() {
    assert_eq!(
        Solution::can_reach("01101110".to_string(), 2, 3),
        false
    );
}
