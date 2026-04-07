// Tests for Problem 2527: Find XOR Beauty of Array
// Java reference: src/test/java/g2401_2500/s2527_find_xor_beauty_of_array/SolutionTest.java

use leetcode_in_rust::s2527::find_xor_beauty_of_array::Solution;

#[test]
fn test_xor_beauty() {
    assert_eq!(Solution::xor_beauty(vec![1, 4]), 5);
}

#[test]
fn test_xor_beauty2() {
    assert_eq!(Solution::xor_beauty(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30]), 34);
}
