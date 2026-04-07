// Tests for Problem 1835: Find XOR Sum of All Pairs Bitwise AND
// Java reference: src/test/java/g1801_1900/s1835_find_xor_sum_of_all_pairs_bitwise_and/SolutionTest.java

use leetcode_in_rust::s1835::find_xor_sum_of_all_pairs_bitwise_and::Solution;

#[test]
fn test_get_xor_sum() {
    assert_eq!(Solution::get_xor_sum(vec![1, 2, 3], vec![6, 5]), 0);
}

#[test]
fn test_get_xor_sum2() {
    assert_eq!(Solution::get_xor_sum(vec![12], vec![4]), 4);
}
