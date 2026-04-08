// Tests for Problem 3240: Minimum Number of Flips to Make Binary Grid Palindromic II
// Java reference: src/test/java/g3201_3300/s3240_minimum_number_of_flips_to_make_binary_grid_palindromic_ii/SolutionTest.java

use leetcode_in_rust::s3240::minimum_number_of_flips_to_make_binary_grid_palindromic_ii::Solution;

#[test]
fn test_min_flips() {
    assert_eq!(
        Solution::min_flips(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
}

#[test]
fn test_min_flips2() {
    assert_eq!(Solution::min_flips(vec![vec![0, 1], vec![0, 1], vec![0, 0]]), 2);
}

#[test]
fn test_min_flips3() {
    assert_eq!(Solution::min_flips(vec![vec![1], vec![1]]), 2);
}
