// Tests for Problem 1720: Decode XORed Array
// Java reference: src/test/java/g1701_1800/s1720_decode_xored_array/SolutionTest.java

use leetcode_in_rust::s1720::decode_xored_array::Solution;

#[test]
fn test_decode() {
    assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
}

#[test]
fn test_decode2() {
    assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
}
