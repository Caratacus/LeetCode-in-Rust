// Tests for Problem 2225: Find Players With Zero or One Losses
// Java reference: src/test/java/g2201_2300/s2225_find_players_with_zero_or_one_losses/SolutionTest.java

use leetcode_in_rust::s2225::find_players_with_zero_or_one_losses::Solution;

#[test]
fn test_find_winners() {
    let result = Solution::find_winners(vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9],
    ]);
    assert_eq!(result[0], vec![1, 2, 10]);
    assert_eq!(result[1], vec![4, 5, 7, 8]);
}

#[test]
fn test_find_winners2() {
    let result = Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]]);
    assert_eq!(result[0], vec![1, 2, 5, 6]);
    assert_eq!(result[1], Vec::<i32>::new());
}
