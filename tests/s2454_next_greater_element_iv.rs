// Tests for Problem 2454: Next Greater Element IV
// Java reference: src/test/java/g2401_2500/s2454_next_greater_element_iv/SolutionTest.java

use leetcode_in_rust::s2454::next_greater_element_iv::Solution;

#[test]
fn test_second_greater_element() {
    assert_eq!(
        Solution::second_greater_element(vec![2, 4, 0, 9, 6]),
        vec![9, 6, 6, -1, -1]
    );
}

#[test]
fn test_second_greater_element2() {
    assert_eq!(Solution::second_greater_element(vec![3, 3]), vec![-1, -1]);
}
