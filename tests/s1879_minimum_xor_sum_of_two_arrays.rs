// Tests for Problem 1879: Minimum XOR Sum of Two Arrays
// Java reference: src/test/java/g1801_1900/s1879_minimum_xor_sum_of_two_arrays/SolutionTest.java

use leetcode_in_rust::s1879::minimum_xor_sum_of_two_arrays::Solution;

#[test]
fn test_minimum_xor_sum() {
    assert_eq!(Solution::minimum_xor_sum(vec![1, 2], vec![2, 3]), 2);
}

#[test]
fn test_minimum_xor_sum2() {
    assert_eq!(Solution::minimum_xor_sum(vec![1, 0, 3], vec![5, 3, 4]), 8);
}
