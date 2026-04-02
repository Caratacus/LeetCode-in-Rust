// Tests for Problem 0190: Reverse Bits
// Java reference: src/test/java/g0181_0200/s0190_reverse_bits/SolutionTest.java

use leetcode_in_rust::s0190::reverse_bits::Solution;

#[test]
fn test_reverse_bits() {
    // 43261596 (00000010100101000001111010011100) reversed -> 964176192
    assert_eq!(Solution::reverse_bits(43261596), 964176192);
}
