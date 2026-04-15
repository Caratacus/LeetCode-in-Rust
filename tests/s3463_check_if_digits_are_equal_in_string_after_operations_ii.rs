// Tests for Problem 3463: Check if Digits Are Equal in String After Operations II
// Java reference: src/test/java/g3401_3500/s3463_check_if_digits_are_equal_in_string_after_operations_ii/SolutionTest.java

use leetcode_in_rust::s3463::check_if_digits_are_equal_in_string_after_operations_ii::Solution;

#[test]
fn test_has_same_digits() {
    assert_eq!(Solution::has_same_digits("3902".to_string()), true);
}

#[test]
fn test_has_same_digits2() {
    assert_eq!(Solution::has_same_digits("34789".to_string()), false);
}

#[test]
fn test_has_same_digits3() {
    assert_eq!(Solution::has_same_digits("3506677".to_string()), false);
}
