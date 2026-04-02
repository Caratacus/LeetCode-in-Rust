// Tests for Problem 0440: K-th Smallest in Lexicographical Order
// Java reference: src/test/java/g0401_0500/s0440_k_th_smallest_in_lexicographical_order/SolutionTest.java

use leetcode_in_rust::s0440::k_th_smallest_in_lexicographical_order::Solution;

#[test]
fn test_find_kth_number() {
    assert_eq!(Solution::find_kth_number(13, 2), 10);
}

#[test]
fn test_find_kth_number2() {
    assert_eq!(Solution::find_kth_number(1, 1), 1);
}
