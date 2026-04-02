// Tests for Problem 0442: Find All Duplicates in an Array
// Java reference: src/test/java/g0401_0500/s0442_find_all_duplicates_in_an_array/SolutionTest.java

use leetcode_in_rust::s0442::find_all_duplicates_in_an_array::Solution;
use std::collections::HashSet;

#[test]
fn test_find_duplicates() {
    let result = Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]);
    let result_set: HashSet<i32> = result.into_iter().collect();
    let expected: HashSet<i32> = vec![2, 3].into_iter().collect();
    assert_eq!(result_set, expected);
}

#[test]
fn test_find_duplicates2() {
    let result = Solution::find_duplicates(vec![1, 1, 2]);
    let result_set: HashSet<i32> = result.into_iter().collect();
    let expected: HashSet<i32> = vec![1].into_iter().collect();
    assert_eq!(result_set, expected);
}
