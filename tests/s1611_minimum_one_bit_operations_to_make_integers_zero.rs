// Tests for Problem 1611: Minimum One Bit Operations to Make Integers Zero
// Java reference: src/test/java/g1601_1700/s1611_minimum_one_bit_operations_to_make_integers_zero/SolutionTest.java

use leetcode_in_rust::s1611::minimum_one_bit_operations_to_make_integers_zero::Solution;

#[test]
fn test_minimum_one_bit_operations() {
    assert_eq!(Solution::minimum_one_bit_operations(3), 2);
}

#[test]
fn test_minimum_one_bit_operations2() {
    assert_eq!(Solution::minimum_one_bit_operations(6), 4);
}
