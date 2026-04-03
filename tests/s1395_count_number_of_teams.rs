// Tests for Problem 1395: Count Number of Teams
// Java reference: src/test/java/g1301_1400/s1395_count_number_of_teams/SolutionTest.java

use leetcode_in_rust::s1395::count_number_of_teams::Solution;

#[test]
fn test_num_teams() {
    assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
}

#[test]
fn test_num_teams2() {
    assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
}

#[test]
fn test_num_teams3() {
    assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
}
