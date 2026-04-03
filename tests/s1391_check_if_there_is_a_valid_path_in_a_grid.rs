// Tests for Problem 1391: Check if There is a Valid Path in a Grid
// Java reference: src/test/java/g1301_1400/s1391_check_if_there_is_a_valid_path_in_a_grid/SolutionTest.java

use leetcode_in_rust::s1391::check_if_there_is_a_valid_path_in_a_grid::Solution;

#[test]
fn test_has_valid_path() {
    assert_eq!(
        Solution::has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]),
        true
    );
}

#[test]
fn test_has_valid_path2() {
    assert_eq!(
        Solution::has_valid_path(vec![vec![1, 2, 1], vec![1, 2, 1]]),
        false
    );
}

#[test]
fn test_has_valid_path3() {
    assert_eq!(
        Solution::has_valid_path(vec![vec![1, 1, 2]]),
        false
    );
}
