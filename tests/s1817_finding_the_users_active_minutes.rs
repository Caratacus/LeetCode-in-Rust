// Tests for Problem 1817: Finding the Users Active Minutes
// Java reference: src/test/java/g1801_1900/s1817_finding_the_users_active_minutes/SolutionTest.java

use leetcode_in_rust::s1817::finding_the_users_active_minutes::Solution;

#[test]
fn test_finding_users_active_minutes() {
    assert_eq!(
        Solution::finding_users_active_minutes(
            vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]],
            5
        ),
        vec![0, 2, 0, 0, 0]
    );
}

#[test]
fn test_finding_users_active_minutes2() {
    assert_eq!(
        Solution::finding_users_active_minutes(vec![vec![1, 1], vec![2, 2], vec![2, 3]], 4),
        vec![1, 1, 0, 0]
    );
}
