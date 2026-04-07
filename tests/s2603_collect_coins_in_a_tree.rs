// Tests for Problem 2603: Collect Coins in a Tree
// Java reference: src/test/java/g2601_2700/s2603_collect_coins_in_a_tree/SolutionTest.java

use leetcode_in_rust::s2603::collect_coins_in_a_tree::Solution;

#[test]
fn test_collect_the_coins() {
    assert_eq!(
        Solution::collect_the_coins(
            vec![1, 0, 0, 0, 0, 1],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]
        ),
        2
    );
}
