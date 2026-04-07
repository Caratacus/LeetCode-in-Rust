// Tests for Problem 2028: Find Missing Observations
// Java reference: src/test/java/g2001_2100/s2028_find_missing_observations/SolutionTest.java

use leetcode_in_rust::s2028::find_missing_observations::Solution;

#[test]
fn test_missing_rolls() {
    assert_eq!(Solution::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
}

#[test]
fn test_missing_rolls2() {
    assert_eq!(Solution::missing_rolls(vec![1, 5, 6], 3, 4), vec![6, 1, 1, 1]);
}

#[test]
fn test_missing_rolls3() {
    assert_eq!(Solution::missing_rolls(vec![1, 2, 3, 4], 6, 4), vec![]);
}
