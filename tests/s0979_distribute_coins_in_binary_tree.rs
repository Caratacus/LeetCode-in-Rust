// Tests for Problem 0979: Distribute Coins in Binary Tree
// Java reference: src/test/java/g0901_1000/s0979_distribute_coins_in_binary_tree/SolutionTest.java

use leetcode_in_rust::s0979::distribute_coins_in_binary_tree::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_distribute_coins() {
    let root = tree_from_vec(vec![Some(3), Some(0), Some(0)]);
    let result = Solution::distribute_coins(root);
    assert_eq!(result, 2);
}

#[test]
fn test_distribute_coins2() {
    let root = tree_from_vec(vec![Some(0), Some(3), Some(0)]);
    let result = Solution::distribute_coins(root);
    assert_eq!(result, 3);
}

#[test]
fn test_distribute_coins3() {
    let root = tree_from_vec(vec![Some(1), Some(0), Some(0), None, Some(3)]);
    let result = Solution::distribute_coins(root);
    assert_eq!(result, 4);
}
