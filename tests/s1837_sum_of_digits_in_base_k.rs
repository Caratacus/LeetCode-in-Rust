// Tests for Problem 1837: Sum of Digits in Base K
// Java reference: src/test/java/g1801_1900/s1837_sum_of_digits_in_base_k/SolutionTest.java

use leetcode_in_rust::s1837::sum_of_digits_in_base_k::Solution;

#[test]
fn test_sum_base() {
    assert_eq!(Solution::sum_base(34, 6), 9);
}

#[test]
fn test_sum_base2() {
    assert_eq!(Solution::sum_base(10, 10), 1);
}
