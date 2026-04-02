// Tests for Problem 0503: Next Greater Element II
// Java reference: src/test/java/g0501_0600/s0503_next_greater_element_ii/SolutionTest.java

use leetcode_in_rust::s0503::next_greater_element_ii::Solution;

#[test]
fn test_next_greater_elements() {
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 1]),
        vec![2, -1, 2]
    );
}

#[test]
fn test_next_greater_elements2() {
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
        vec![2, 3, 4, -1, 4]
    );
}
