// Tests for Problem 2151: Maximum Good People Based on Statements
// Java reference: src/test/java/g2101_2200/s2151_maximum_good_people_based_on_statements/SolutionTest.java

use leetcode_in_rust::s2151::maximum_good_people_based_on_statements::Solution;

#[test]
fn test_maximum_good() {
    assert_eq!(
        Solution::maximum_good(vec![vec![2, 1, 2], vec![1, 2, 2], vec![2, 0, 2]]),
        2
    );
}

#[test]
fn test_maximum_good2() {
    assert_eq!(Solution::maximum_good(vec![vec![2, 0], vec![0, 2]]), 1);
}
