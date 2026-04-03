// Tests for Problem 1563: Stone Game V
// Java reference: src/test/java/g1501_1600/s1563_stone_game_v/SolutionTest.java

use leetcode_in_rust::s1563::stone_game_v::Solution;

#[test]
fn test_stone_game_v() {
    assert_eq!(Solution::stone_game_v(vec![6, 2, 3, 4, 5, 5]), 18);
}

#[test]
fn test_stone_game_v2() {
    assert_eq!(Solution::stone_game_v(vec![7, 7, 7, 7, 7, 7, 7]), 28);
}

#[test]
fn test_stone_game_v3() {
    assert_eq!(Solution::stone_game_v(vec![4]), 0);
}
