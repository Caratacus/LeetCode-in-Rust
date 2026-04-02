// Tests for Problem 0496: Next Greater Element I
// Java reference: src/test/java/g0401_0500/s0496_next_greater_element_i/SolutionTest.java

use leetcode_in_rust::s0496::next_greater_element_i::Solution;

#[test]
fn test_next_greater_element() {
    assert_eq!(
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
}

#[test]
fn test_next_greater_element2() {
    assert_eq!(
        Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
