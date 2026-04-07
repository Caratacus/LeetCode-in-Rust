// Tests for Problem 1846: Maximum Element After Decreasing and Rearranging
// Java reference: src/test/java/g1801_1900/s1846_maximum_element_after_decreasing_and_rearranging/SolutionTest.java

use leetcode_in_rust::s1846::maximum_element_after_decreasing_and_rearranging::Solution;

#[test]
fn test_maximum_element_after_decrementing_and_rearranging() {
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
        2
    );
}

#[test]
fn test_maximum_element_after_decrementing_and_rearranging2() {
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
        3
    );
}

#[test]
fn test_maximum_element_after_decrementing_and_rearranging3() {
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
        5
    );
}
