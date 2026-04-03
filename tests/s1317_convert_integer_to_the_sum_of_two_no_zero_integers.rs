// Tests for Problem 1317: Convert Integer to the Sum of Two No-Zero Integers
// Java reference: src/test/java/g1301_1400/s1317_convert_integer_to_the_sum_of_two_no_zero_integers/SolutionTest.java

use leetcode_in_rust::s1317::convert_integer_to_the_sum_of_two_no_zero_integers::Solution;

#[test]
fn test_get_no_zero_integers() {
    let result = Solution::get_no_zero_integers(2);
    assert_eq!(result[0] + result[1], 2);
    assert!(result[0] > 0 && result[1] > 0);
}

#[test]
fn test_get_no_zero_integers2() {
    let result = Solution::get_no_zero_integers(11);
    assert_eq!(result[0] + result[1], 11);
    assert!(result[0] > 0 && result[1] > 0);
}
