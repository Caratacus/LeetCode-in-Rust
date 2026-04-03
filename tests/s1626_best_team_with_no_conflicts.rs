// Tests for Problem 1626: Best Team With No Conflicts
// Java reference: src/test/java/g1601_1700/s1626_best_team_with_no_conflicts/SolutionTest.java

use leetcode_in_rust::s1626::best_team_with_no_conflicts::Solution;

#[test]
fn test_best_team_score() {
    assert_eq!(Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]), 34);
}

#[test]
fn test_best_team_score2() {
    assert_eq!(Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]), 16);
}

#[test]
fn test_best_team_score3() {
    assert_eq!(Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]), 6);
}
