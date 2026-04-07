// Tests for Problem 1900: The Earliest and Latest Rounds Where Players Compete
// Java reference: src/test/java/g1901_2000/s1900_the_earliest_and_latest_rounds_where_players_compete/SolutionTest.java

use leetcode_in_rust::s1900::the_earliest_and_latest_rounds_where_players_compete::Solution;

#[test]
fn test_earliest_and_latest() {
    assert_eq!(Solution::earliest_and_latest(11, 2, 4), vec![3, 4]);
}

#[test]
fn test_earliest_and_latest2() {
    assert_eq!(Solution::earliest_and_latest(5, 1, 5), vec![1, 1]);
}
