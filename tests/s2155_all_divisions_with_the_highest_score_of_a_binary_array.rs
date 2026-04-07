// Tests for Problem 2155: All Divisions With the Highest Score of a Binary Array
// Java reference: src/test/java/g2101_2200/s2155_all_divisions_with_the_highest_score_of_a_binary_array/SolutionTest.java

use leetcode_in_rust::s2155::all_divisions_with_the_highest_score_of_a_binary_array::Solution;

#[test]
fn test_max_score_indices() {
    let mut result = Solution::max_score_indices(vec![0, 0, 1, 0]);
    result.sort();
    assert_eq!(result, vec![2, 4]);
}

#[test]
fn test_max_score_indices2() {
    assert_eq!(Solution::max_score_indices(vec![0, 0, 0]), vec![3]);
}

#[test]
fn test_max_score_indices3() {
    assert_eq!(Solution::max_score_indices(vec![1, 1]), vec![0]);
}
