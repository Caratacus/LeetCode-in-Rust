// Tests for Problem 0448: Find All Numbers Disappeared in an Array
// Java reference: src/test/java/g0401_0500/s0448_find_all_numbers_disappeared_in_an_array/SolutionTest.java

use leetcode_in_rust::s0448::find_all_numbers_disappeared_in_an_array::Solution;
use std::collections::HashSet;

#[test]
fn test_find_disappeared_numbers() {
    let result = Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]);
    let result_set: HashSet<i32> = result.into_iter().collect();
    let expected: HashSet<i32> = vec![5, 6].into_iter().collect();
    assert_eq!(result_set, expected);
}

#[test]
fn test_find_disappeared_numbers2() {
    let result = Solution::find_disappeared_numbers(vec![1, 1]);
    let result_set: HashSet<i32> = result.into_iter().collect();
    let expected: HashSet<i32> = vec![2].into_iter().collect();
    assert_eq!(result_set, expected);
}
