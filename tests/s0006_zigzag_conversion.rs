// Tests for Problem 0006: Zigzag Conversion
// Java reference: src/test/java/g0001_0100/s0006_zigzag_conversion/SolutionTest.java

use leetcode_in_rust::s0006::zigzag_conversion::Solution;

#[test]
fn test_convert() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
}

#[test]
fn test_convert2() {
    assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
}
