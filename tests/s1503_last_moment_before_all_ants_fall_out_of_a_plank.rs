// Tests for Problem 1503: Last Moment Before All Ants Fall Out of a Plank
// Java reference: src/test/java/g1501_1600/s1503_last_moment_before_all_ants_fall_out_of_a_plank/SolutionTest.java

use leetcode_in_rust::s1503::last_moment_before_all_ants_fall_out_of_a_plank::Solution;

#[test]
fn test_get_last_moment() {
    assert_eq!(Solution::get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
}

#[test]
fn test_get_last_moment2() {
    assert_eq!(Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]), 7);
}

#[test]
fn test_get_last_moment3() {
    assert_eq!(Solution::get_last_moment(7, vec![0, 1, 2, 3, 4, 5, 6, 7], vec![]), 7);
}
