// Tests for Problem 0421: Maximum XOR of Two Numbers in an Array
// Java reference: src/test/java/g0401_0500/s0421_maximum_xor_of_two_numbers_in_an_array/SolutionTest.java

use leetcode_in_rust::s0421::maximum_xor_of_two_numbers_in_an_array::Solution;

#[test]
fn test_find_maximum_xor() {
    assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
}

#[test]
fn test_find_maximum_xor2() {
    assert_eq!(Solution::find_maximum_xor(vec![0]), 0);
}

#[test]
fn test_find_maximum_xor3() {
    assert_eq!(Solution::find_maximum_xor(vec![2, 4]), 6);
}
