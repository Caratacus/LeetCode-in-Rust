// Tests for Problem 1366: Rank Teams by Votes
// Java reference: src/test/java/g1301_1400/s1366_rank_teams_by_votes/SolutionTest.java

use leetcode_in_rust::s1366::rank_teams_by_votes::Solution;

#[test]
fn test_rank_teams() {
    let result = Solution::rank_teams(vec!["ABC".to_string(), "ACB".to_string(), "ABC".to_string(), "ACB".to_string(), "ACB".to_string()]);
    assert_eq!(result, "ACB");
}

#[test]
fn test_rank_teams2() {
    let result = Solution::rank_teams(vec!["WXYZ".to_string(), "XYZW".to_string()]);
    assert_eq!(result, "XWYZ");
}

#[test]
fn test_rank_teams3() {
    let result = Solution::rank_teams(vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()]);
    assert_eq!(result, "ZMNAGUEDSJYLBOPHRQICWFXTVK");
}
