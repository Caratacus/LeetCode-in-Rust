// Tests for Problem 3376: Minimum Time to Break Locks I
// Java reference: src/test/java/g3301_3400/s3376_minimum_time_to_break_locks_i/SolutionTest.java

use leetcode_in_rust::s3376::minimum_time_to_break_locks_i::Solution;

#[test]
fn test_find_minimum_time() {
    assert_eq!(Solution::find_minimum_time(vec![3, 4, 1], 1), 4);
}

#[test]
fn test_find_minimum_time2() {
    assert_eq!(Solution::find_minimum_time(vec![2, 5, 4], 2), 5);
}
