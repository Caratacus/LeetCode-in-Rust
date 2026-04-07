// Tests for Problem 2491: Divide Players Into Teams of Equal Skill
// Java reference: src/test/java/g2401_2500/s2491_divide_players_into_teams_of_equal_skill/SolutionTest.java

use leetcode_in_rust::s2491::divide_players_into_teams_of_equal_skill::Solution;

#[test]
fn test_divide_players() {
    assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
}

#[test]
fn test_divide_players2() {
    assert_eq!(Solution::divide_players(vec![3, 4]), 12);
}

#[test]
fn test_divide_players3() {
    assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
}
