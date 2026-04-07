// Tests for Problem 2032: Two Out of Three
// Java reference: src/test/java/g2001_2100/s2032_two_out_of_three/SolutionTest.java

use leetcode_in_rust::s2032::two_out_of_three::Solution;
use std::collections::HashSet;

#[test]
fn test_two_out_of_three() {
    let result = Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]);
    let result_set: HashSet<i32> = result.into_iter().collect();
    let expected_set: HashSet<i32> = vec![2, 3].into_iter().collect();
    assert_eq!(result_set, expected_set);
}

#[test]
fn test_two_out_of_three2() {
    let result = Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]);
    let result_set: HashSet<i32> = result.into_iter().collect();
    let expected_set: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(result_set, expected_set);
}

#[test]
fn test_two_out_of_three3() {
    let result = Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]);
    assert_eq!(result, vec![]);
}
