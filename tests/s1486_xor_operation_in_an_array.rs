// Tests for Problem 1486: XOR Operation in an Array
// Java reference: src/test/java/g1401_1500/s1486_xor_operation_in_an_array/SolutionTest.java

use leetcode_in_rust::s1486::xor_operation_in_an_array::Solution;

#[test]
fn test_xor_operation() {
    assert_eq!(Solution::xor_operation(5, 0), 8);
}

#[test]
fn test_xor_operation2() {
    assert_eq!(Solution::xor_operation(4, 3), 8);
}
