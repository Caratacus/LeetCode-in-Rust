// Tests for Problem 1340: Jump Game V
// Java reference: src/test/java/g1301_1400/s1340_jump_game_v/SolutionTest.java

use leetcode_in_rust::s1340::jump_game_v::Solution;

#[test]
fn test_max_jumps() {
    assert_eq!(Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
}

#[test]
fn test_max_jumps2() {
    assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
}

#[test]
fn test_max_jumps3() {
    assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
}
