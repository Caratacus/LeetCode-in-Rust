// Tests for Problem 3394: Check if Grid Can Be Cut Into Sections
// Java reference: src/test/java/g3301_3400/s3394_check_if_grid_can_be_cut_into_sections/SolutionTest.java

use leetcode_in_rust::s3394::check_if_grid_can_be_cut_into_sections::Solution;

#[test]
fn test_check_valid_cuts() {
    assert!(Solution::check_valid_cuts(
        5,
        vec![vec![1, 0, 5, 2], vec![0, 2, 2, 4], vec![3, 2, 5, 3], vec![0, 4, 4, 5]]
    ));
}

#[test]
fn test_check_valid_cuts2() {
    assert!(Solution::check_valid_cuts(
        4,
        vec![vec![0, 0, 1, 1], vec![2, 0, 3, 4], vec![0, 2, 2, 3], vec![3, 0, 4, 3]]
    ));
}

#[test]
fn test_check_valid_cuts3() {
    assert!(!Solution::check_valid_cuts(
        4,
        vec![
            vec![0, 2, 2, 4],
            vec![1, 0, 3, 2],
            vec![2, 2, 3, 4],
            vec![3, 0, 4, 2],
            vec![3, 2, 4, 4]
        ]
    ));
}
