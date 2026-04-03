// Tests for Problem 1785: Minimum Elements to Add to Form a Given Sum
// Java reference: src/test/java/g1701_1800/s1785_minimum_elements_to_add_to_form_a_given_sum/SolutionTest.java

use leetcode_in_rust::s1785::minimum_elements_to_add_to_form_a_given_sum::Solution;

#[test]
fn test_min_elements() {
    assert_eq!(Solution::min_elements(vec![1, -1, 1], 3, -4), 2);
}

#[test]
fn test_min_elements2() {
    assert_eq!(Solution::min_elements(vec![1, -10, 9, 1], 100, 0), 1);
}
