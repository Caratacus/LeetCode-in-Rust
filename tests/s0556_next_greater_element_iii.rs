// Tests for Problem 0556: Next Greater Element III
// Java reference: src/test/java/g0501_0600/s0556_next_greater_element_iii/SolutionTest.java

use leetcode_in_rust::s0556::next_greater_element_iii::Solution;

#[test]
fn test_next_greater_element() {
    assert_eq!(Solution::next_greater_element(12), 21);
}

#[test]
fn test_next_greater_element2() {
    assert_eq!(Solution::next_greater_element(21), -1);
}

#[test]
fn test_next_greater_element3() {
    assert_eq!(Solution::next_greater_element(1234), 1243);
}
