// Tests for Problem 1492: The kth Factor of n
// Java reference: src/test/java/g1401_1500/s1492_the_kth_factor_of_n/SolutionTest.java

use leetcode_in_rust::s1492::the_kth_factor_of_n::Solution;

#[test]
fn test_kth_factor() {
    assert_eq!(Solution::kth_factor(12, 3), 3);
}

#[test]
fn test_kth_factor2() {
    assert_eq!(Solution::kth_factor(7, 2), 7);
}

#[test]
fn test_kth_factor3() {
    assert_eq!(Solution::kth_factor(4, 4), -1);
}
