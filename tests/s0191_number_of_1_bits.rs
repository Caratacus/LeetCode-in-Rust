// Tests for Problem 0191: Number of 1 Bits
// Java reference: src/test/java/g0181_0200/s0191_number_of_1_bits/SolutionTest.java

use leetcode_in_rust::s0191::number_of_1_bits::Solution;

#[test]
fn test_hamming_weight() {
    assert_eq!(Solution::hamming_weight(11), 3);
}

#[test]
fn test_hamming_weight2() {
    assert_eq!(Solution::hamming_weight(128), 1);
}

#[test]
fn test_hamming_weight3() {
    assert_eq!(Solution::hamming_weight(2147483645), 30);
}
