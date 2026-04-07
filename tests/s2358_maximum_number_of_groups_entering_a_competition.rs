// Tests for Problem 2358: Maximum Number of Groups Entering a Competition
// Java reference: src/test/java/g2301_2400/s2358_maximum_number_of_groups_entering_a_competition/SolutionTest.java

use leetcode_in_rust::s2358::maximum_number_of_groups_entering_a_competition::Solution;

#[test]
fn test_maximum_groups() {
    assert_eq!(Solution::maximum_groups(vec![10, 6, 12, 7, 3, 5]), 3);
}

#[test]
fn test_maximum_groups2() {
    assert_eq!(Solution::maximum_groups(vec![8, 8]), 1);
}
