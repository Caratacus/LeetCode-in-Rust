// Tests for Problem 1035: Uncrossed Lines
// Java reference: src/test/java/g1001_1100/s1035_uncrossed_lines/SolutionTest.java

use leetcode_in_rust::s1035::uncrossed_lines::Solution;

#[test]
fn test_max_uncrossed_lines() {
    assert_eq!(Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
}

#[test]
fn test_max_uncrossed_lines2() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
}

#[test]
fn test_max_uncrossed_lines3() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}
