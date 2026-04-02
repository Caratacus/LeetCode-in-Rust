// Tests for Problem 1145: Binary Tree Coloring Game
// Java reference: src/test/java/g1101_1200/s1145_binary_tree_coloring_game/SolutionTest.java

use leetcode_in_rust::s1145::binary_tree_coloring_game::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_btree_game_winning_move() {
    let root = tree_from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
        Some(10),
        Some(11),
    ]);
    assert_eq!(Solution::btree_game_winning_move(root, 11, 3), true);
}

#[test]
fn test_btree_game_winning_move2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::btree_game_winning_move(root, 3, 1), false);
}
