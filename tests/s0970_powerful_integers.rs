// Tests for Problem 0970: Powerful Integers
// Java reference: src/test/java/g0901_1000/s0970_powerful_integers/SolutionTest.java

use leetcode_in_rust::s0970::powerful_integers::Solution;
use std::collections::HashSet;

fn sorted_unique(vec: Vec<i32>) -> Vec<i32> {
    let set: HashSet<i32> = vec.into_iter().collect();
    let mut result: Vec<i32> = set.into_iter().collect();
    result.sort();
    result
}

#[test]
fn test_powerful_integers() {
    let result = Solution::powerful_integers(2, 3, 10);
    let mut expected = vec![2, 3, 4, 5, 7, 9, 10];
    expected.sort();
    assert_eq!(sorted_unique(result), expected);
}

#[test]
fn test_powerful_integers2() {
    let result = Solution::powerful_integers(3, 5, 15);
    let mut expected = vec![2, 4, 6, 8, 10, 14];
    expected.sort();
    assert_eq!(sorted_unique(result), expected);
}
