// Tests for Problem 1449: Form Largest Integer With Digits That Add up to Target
// Java reference: src/test/java/g1401_1500/s1449_form_largest_integer_with_digits_that_add_up_to_target/SolutionTest.java

use leetcode_in_rust::s1449::form_largest_integer_with_digits_that_add_up_to_target::Solution;

#[test]
fn test_largest_number() {
    assert_eq!(Solution::largest_number(vec![4, 3, 2, 5, 6, 7, 2, 5, 5], 9), "7772");
}

#[test]
fn test_largest_number2() {
    assert_eq!(Solution::largest_number(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12), "85");
}

#[test]
fn test_largest_number3() {
    assert_eq!(Solution::largest_number(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5), "0");
}
