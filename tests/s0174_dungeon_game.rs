// Tests for Problem 0174: Dungeon Game
// Java reference: src/test/java/g0121_0200/s0174_dungeon_game/SolutionTest.java

use leetcode_in_rust::s0174::dungeon_game::Solution;

#[test]
fn test_calculate_minimum_hp() {
    let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    assert_eq!(Solution::calculate_minimum_hp(dungeon), 7);
}
