// Tests for Problem 2595: Number of Even and Odd Bits
// Java reference: src/test/java/g2501_2600/s2595_number_of_even_and_odd_bits/SolutionTest.java

use leetcode_in_rust::s2595::number_of_even_and_odd_bits::Solution;

#[test]
fn test_even_odd_bit() {
    assert_eq!(Solution::even_odd_bit(17), vec![2, 0]);
}

#[test]
fn test_even_odd_bit2() {
    assert_eq!(Solution::even_odd_bit(2), vec![0, 1]);
}
