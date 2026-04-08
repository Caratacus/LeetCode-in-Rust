// Tests for Problem 3238: Find the Number of Winning Players
// Java reference: src/test/java/g3201_3300/s3238_find_the_number_of_winning_players/SolutionTest.java

use leetcode_in_rust::s3238::find_the_number_of_winning_players::Solution;

#[test]
fn test_winning_player_count() {
    assert_eq!(
        Solution::winning_player_count(
            4,
            vec![vec![0, 0], vec![1, 0], vec![1, 0], vec![2, 1], vec![2, 1], vec![2, 0]]
        ),
        2
    );
}

#[test]
fn test_winning_player_count2() {
    assert_eq!(
        Solution::winning_player_count(5, vec![vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4]]),
        0
    );
}

#[test]
fn test_winning_player_count3() {
    assert_eq!(
        Solution::winning_player_count(5, vec![vec![1, 1], vec![2, 4], vec![2, 4], vec![2, 4]]),
        1
    );
}
