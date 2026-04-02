// Tests for Problem 0989: Add to Array-Form of Integer
// Java reference: src/test/java/g0901_1000/s0989_add_to_array_form_of_integer/SolutionTest.java

use leetcode_in_rust::s0989::add_to_array_form_of_integer::Solution;

#[test]
fn test_add_to_array_form() {
    assert_eq!(Solution::add_to_array_form(vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]);
}

#[test]
fn test_add_to_array_form2() {
    assert_eq!(Solution::add_to_array_form(vec![2, 7, 4], 181), vec![4, 5, 5]);
}

#[test]
fn test_add_to_array_form3() {
    assert_eq!(Solution::add_to_array_form(vec![2, 1, 5], 806), vec![1, 0, 2, 1]);
}
