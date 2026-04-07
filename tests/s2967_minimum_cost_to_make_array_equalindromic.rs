// Tests for Problem 2967: Minimum Cost to Make Array Equalindromic
// Java reference: src/test/java/g2901_3000/s2967_minimum_cost_to_make_array_equalindromic/SolutionTest.java

use leetcode_in_rust::s2967::minimum_cost_to_make_array_equalindromic::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 4, 5]), 6);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(Solution::minimum_cost(vec![10, 12, 13, 14, 15]), 11);
}

#[test]
fn test_minimum_cost3() {
    assert_eq!(Solution::minimum_cost(vec![22, 33, 22, 33, 22]), 22);
}
