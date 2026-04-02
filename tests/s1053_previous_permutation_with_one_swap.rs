// Tests for Problem 1053: Previous Permutation With One Swap
// Java reference: src/test/java/g1001_1100/s1053_previous_permutation_with_one_swap/SolutionTest.java

use leetcode_in_rust::s1053::previous_permutation_with_one_swap::Solution;

#[test]
fn test_prev_perm_opt1() {
    assert_eq!(Solution::prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
}

#[test]
fn test_prev_perm_opt2() {
    assert_eq!(Solution::prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
}

#[test]
fn test_prev_perm_opt3() {
    assert_eq!(Solution::prev_perm_opt1(vec![1, 9, 4, 6, 7]), vec![1, 7, 4, 6, 9]);
}
