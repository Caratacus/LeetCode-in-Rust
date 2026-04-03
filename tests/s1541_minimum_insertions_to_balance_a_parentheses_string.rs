// Tests for Problem 1541: Minimum Insertions to Balance a Parentheses String
// Java reference: src/test/java/g1501_1600/s1541_minimum_insertions_to_balance_a_parentheses_string/SolutionTest.java

use leetcode_in_rust::s1541::minimum_insertions_to_balance_a_parentheses_string::Solution;

#[test]
fn test_min_insertions() {
    assert_eq!(Solution::min_insertions("(()))".to_string()), 1);
}

#[test]
fn test_min_insertions2() {
    assert_eq!(Solution::min_insertions("())".to_string()), 0);
}

#[test]
fn test_min_insertions3() {
    assert_eq!(Solution::min_insertions("))())(".to_string()), 3);
}
