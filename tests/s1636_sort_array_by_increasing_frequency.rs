// Tests for Problem 1636: Sort Array by Increasing Frequency
// Java reference: src/test/java/g1601_1700/s1636_sort_array_by_increasing_frequency/SolutionTest.java

use leetcode_in_rust::s1636::sort_array_by_increasing_frequency::Solution;

#[test]
fn test_frequency_sort() {
    assert_eq!(Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]), vec![3, 1, 1, 2, 2, 2]);
}

#[test]
fn test_frequency_sort2() {
    assert_eq!(Solution::frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
}

#[test]
fn test_frequency_sort3() {
    assert_eq!(Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]), vec![5, -1, 4, 4, -6, -6, 1, 1, 1]);
}
