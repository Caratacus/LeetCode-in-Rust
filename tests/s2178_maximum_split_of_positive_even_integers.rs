// Tests for Problem 2178: Maximum Split of Positive Even Integers
// Java reference: src/test/java/g2101_2200/s2178_maximum_split_of_positive_even_integers/SolutionTest.java

use leetcode_in_rust::s2178::maximum_split_of_positive_even_integers::Solution;

#[test]
fn test_maximum_even_split() {
    let mut result = Solution::maximum_even_split(12);
    result.sort();
    assert_eq!(result, vec![2, 4, 6]);
}

#[test]
fn test_maximum_even_split2() {
    assert_eq!(Solution::maximum_even_split(7), vec![]);
}

#[test]
fn test_maximum_even_split3() {
    let mut result = Solution::maximum_even_split(28);
    result.sort();
    assert_eq!(result, vec![2, 4, 6, 16]);
}
