// Tests for Problem 1998: GCD Sort of an Array
// Java reference: src/test/java/g1901_2000/s1998_gcd_sort_of_an_array/SolutionTest.java

use leetcode_in_rust::s1998::gcd_sort_of_an_array::Solution;

#[test]
fn test_gcd_sort() {
    assert_eq!(Solution::gcd_sort(vec![7, 21, 3]), true);
}

#[test]
fn test_gcd_sort2() {
    assert_eq!(Solution::gcd_sort(vec![5, 2, 6, 2]), false);
}

#[test]
fn test_gcd_sort3() {
    assert_eq!(Solution::gcd_sort(vec![10, 5, 9, 3, 15]), true);
}
