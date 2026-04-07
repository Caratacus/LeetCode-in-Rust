// Tests for Problem 2220: Minimum Bit Flips to Convert Number
// Java reference: src/test/java/g2201_2300/s2220_minimum_bit_flips_to_convert_number/SolutionTest.java

use leetcode_in_rust::s2220::minimum_bit_flips_to_convert_number::Solution;

#[test]
fn test_min_bit_flips() {
    assert_eq!(Solution::min_bit_flips(10, 7), 3);
}

#[test]
fn test_min_bit_flips2() {
    assert_eq!(Solution::min_bit_flips(3, 4), 3);
}
