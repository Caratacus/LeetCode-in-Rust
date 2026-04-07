// Tests for Problem 2366: Minimum Replacements to Sort the Array
// Java reference: src/test/java/g2301_2400/s2366_minimum_replacements_to_sort_the_array/SolutionTest.java

use leetcode_in_rust::s2366::minimum_replacements_to_sort_the_array::Solution;

#[test]
fn test_minimum_replacement() {
    assert_eq!(Solution::minimum_replacement(vec![3, 9, 3]), 2);
}

#[test]
fn test_minimum_replacement2() {
    assert_eq!(Solution::minimum_replacement(vec![1, 2, 3, 4, 5]), 0);
}
