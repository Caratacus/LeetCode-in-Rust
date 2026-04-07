// Tests for Problem 2127: Maximum Employees to Be Invited to a Meeting
// Java reference: src/test/java/g2101_2200/s2127_maximum_employees_to_be_invited_to_a_meeting/SolutionTest.java

use leetcode_in_rust::s2127::maximum_employees_to_be_invited_to_a_meeting::Solution;

#[test]
fn test_maximum_invitations() {
    assert_eq!(Solution::maximum_invitations(vec![2, 2, 1, 2]), 3);
}

#[test]
fn test_maximum_invitations2() {
    assert_eq!(Solution::maximum_invitations(vec![1, 2, 0]), 3);
}

#[test]
fn test_maximum_invitations3() {
    assert_eq!(Solution::maximum_invitations(vec![3, 0, 1, 4, 1]), 4);
}
