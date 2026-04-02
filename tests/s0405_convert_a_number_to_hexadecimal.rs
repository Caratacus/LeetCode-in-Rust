// Tests for Problem 0405: Convert a Number to Hexadecimal
// Java reference: src/test/java/g0401_0500/s0405_convert_a_number_to_hexadecimal/SolutionTest.java

use leetcode_in_rust::s0405::convert_a_number_to_hexadecimal::Solution;

#[test]
fn test_to_hex() {
    assert_eq!(Solution::to_hex(26), "1a");
}

#[test]
fn test_to_hex2() {
    assert_eq!(Solution::to_hex(-1), "ffffffff");
}

#[test]
fn test_to_hex3() {
    assert_eq!(Solution::to_hex(0), "0");
}
