// Tests for Problem 0762: Prime Number of Set Bits in Binary Representation
// Java reference: src/test/java/g0701_0800/s0762_prime_number_of_set_bits_in_binary_representation/SolutionTest.java

use leetcode_in_rust::s0762::prime_number_of_set_bits_in_binary_representation::Solution;

#[test]
fn test_count_prime_set_bits() {
    assert_eq!(Solution::count_prime_set_bits(6, 10), 4);
}

#[test]
fn test_count_prime_set_bits2() {
    assert_eq!(Solution::count_prime_set_bits(10, 15), 5);
}
