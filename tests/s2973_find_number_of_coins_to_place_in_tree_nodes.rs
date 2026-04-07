// Tests for Problem 2973: Find Number of Coins to Place in Tree Nodes
// Java reference: src/test/java/g2901_3000/s2973_find_number_of_coins_to_place_in_tree_nodes/SolutionTest.java

use leetcode_in_rust::s2973::find_number_of_coins_to_place_in_tree_nodes::Solution;

#[test]
fn test_placed_coins() {
    assert_eq!(
        Solution::placed_coins(
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]],
            vec![1, 2, 3, 4, 5, 6]
        ),
        vec![120, 1, 1, 1, 1, 1]
    );
}

#[test]
fn test_placed_coins2() {
    assert_eq!(
        Solution::placed_coins(
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 7], vec![2, 8]],
            vec![1, 4, 2, 3, 5, 7, 8, -4, 2]
        ),
        vec![280, 140, 32, 1, 1, 1, 1, 1, 1]
    );
}

#[test]
fn test_placed_coins3() {
    assert_eq!(
        Solution::placed_coins(vec![vec![0, 1], vec![0, 2]], vec![1, 2, -2]),
        vec![0, 1, 1]
    );
}
