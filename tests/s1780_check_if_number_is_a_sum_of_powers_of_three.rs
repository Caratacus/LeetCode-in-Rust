// Tests for Problem 1780: Check if Number is a Sum of Powers of Three
// Java reference: src/test/java/g1701_1800/s1780_check_if_number_is_a_sum_of_powers_of_three/SolutionTest.java

use leetcode_in_rust::s1780::check_if_number_is_a_sum_of_powers_of_three::Solution;

#[test]
fn test_check_powers_of_three() {
    assert_eq!(Solution::check_powers_of_three(12), true);
}

#[test]
fn test_check_powers_of_three2() {
    assert_eq!(Solution::check_powers_of_three(91), true);
}

#[test]
fn test_check_powers_of_three3() {
    assert_eq!(Solution::check_powers_of_three(21), false);
}
