// Tests for Problem 2410: Maximum Matching of Players with Trainers
// Java reference: src/test/java/g2401_2500/s2410_maximum_matching_of_players_with_trainers/SolutionTest.java

use leetcode_in_rust::s2410::maximum_matching_of_players_with_trainers::Solution;

#[test]
fn test_match_players_and_trainers() {
    assert_eq!(
        Solution::match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]),
        2
    );
}

#[test]
fn test_match_players_and_trainers2() {
    assert_eq!(
        Solution::match_players_and_trainers(vec![1, 1, 1], vec![10]),
        1
    );
}
