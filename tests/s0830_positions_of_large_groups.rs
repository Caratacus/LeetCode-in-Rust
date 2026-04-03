// Tests for Problem 0830: Positions of Large Groups
// Java reference: src/test/java/g0701_0800/s0830_positions_of_large_groups/SolutionTest.java

use leetcode_in_rust::s0830::positions_of_large_groups::Solution;

#[test]
fn test_large_group_positions() {
    assert_eq!(
        Solution::large_group_positions("abbxxxxzzy".to_string()),
        vec![vec![3_i32, 6_i32]]
    );
}

#[test]
fn test_large_group_positions2() {
    let result: Vec<Vec<i32>> = Solution::large_group_positions("abc".to_string());
    assert!(result.is_empty());
}

#[test]
fn test_large_group_positions3() {
    assert_eq!(
        Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
        vec![vec![3_i32, 5_i32], vec![6_i32, 9_i32], vec![12_i32, 14_i32]]
    );
}
