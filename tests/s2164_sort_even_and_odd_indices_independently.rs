// Tests for Problem 2164: Sort Even and Odd Indices Independently
// Java reference: src/test/java/g2101_2200/s2164_sort_even_and_odd_indices_independently/SolutionTest.java

use leetcode_in_rust::s2164::sort_even_and_odd_indices_independently::Solution;

#[test]
fn test_sort_even_odd() {
    assert_eq!(Solution::sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
}

#[test]
fn test_sort_even_odd2() {
    assert_eq!(Solution::sort_even_odd(vec![2, 1]), vec![2, 1]);
}
