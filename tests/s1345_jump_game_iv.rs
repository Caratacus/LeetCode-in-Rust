// Tests for Problem 1345: Jump Game IV
// Java reference: src/test/java/g1301_1400/s1345_jump_game_iv/SolutionTest.java

use leetcode_in_rust::s1345::jump_game_iv::Solution;

#[test]
fn test_min_jumps() {
    assert_eq!(
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
}

#[test]
fn test_min_jumps2() {
    assert_eq!(Solution::min_jumps(vec![7]), 0);
}

#[test]
fn test_min_jumps3() {
    assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
}
