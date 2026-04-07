// Tests for Problem 2231: Largest Number After Digit Swaps by Parity
// Java reference: src/test/java/g2201_2300/s2231_largest_number_after_digit_swaps_by_parity/SolutionTest.java

use leetcode_in_rust::s2231::largest_number_after_digit_swaps_by_parity::Solution;

#[test]
fn test_largest_integer() {
    assert_eq!(Solution::largest_integer(1234), 3412);
}

#[test]
fn test_largest_integer2() {
    assert_eq!(Solution::largest_integer(65875), 87655);
}
