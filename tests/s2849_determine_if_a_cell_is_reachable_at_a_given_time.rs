// Tests for Problem 2849: Determine if a Cell Is Reachable at a Given Time
// Java reference: src/test/java/g2801_2900/s2849_determine_if_a_cell_is_reachable_at_a_given_time/SolutionTest.java

use leetcode_in_rust::s2849::determine_if_a_cell_is_reachable_at_a_given_time::Solution;

#[test]
fn test_is_reachable_at_time() {
    assert_eq!(Solution::is_reachable_at_time(2, 4, 7, 7, 6), true);
}

#[test]
fn test_is_reachable_at_time2() {
    assert_eq!(Solution::is_reachable_at_time(3, 1, 7, 3, 3), false);
}

#[test]
fn test_is_reachable_at_time3() {
    assert_eq!(Solution::is_reachable_at_time(3, 1, 3, 1, 3), true);
}

#[test]
fn test_is_reachable_at_time4() {
    assert_eq!(Solution::is_reachable_at_time(3, 1, 3, 1, 1), false);
}
