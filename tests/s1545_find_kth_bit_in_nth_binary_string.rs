// Tests for Problem 1545: Find Kth Bit in Nth Binary String
// Java reference: src/test/java/g1501_1600/s1545_find_kth_bit_in_nth_binary_string/SolutionTest.java

use leetcode_in_rust::s1545::find_kth_bit_in_nth_binary_string::Solution;

#[test]
fn test_find_kth_bit() {
    assert_eq!(Solution::find_kth_bit(3, 1), '0');
}

#[test]
fn test_find_kth_bit2() {
    assert_eq!(Solution::find_kth_bit(4, 11), '1');
}
