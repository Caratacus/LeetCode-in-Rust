// Tests for Problem 3440: Reschedule Meetings for Maximum Free Time II
// Java reference: src/test/java/g3401_3500/s3440_reschedule_meetings_for_maximum_free_time_ii/SolutionTest.java

use leetcode_in_rust::s3440::reschedule_meetings_for_maximum_free_time_ii::Solution;

#[test]
fn test_max_free_time() {
    assert_eq!(Solution::max_free_time(5, vec![1, 3], vec![2, 5]), 2);
}

#[test]
fn test_max_free_time2() {
    assert_eq!(Solution::max_free_time(10, vec![0, 7, 9], vec![1, 8, 10]), 7);
}

#[test]
fn test_max_free_time3() {
    assert_eq!(Solution::max_free_time(10, vec![0, 3, 7, 9], vec![1, 4, 8, 10]), 6);
}

#[test]
fn test_max_free_time4() {
    assert_eq!(Solution::max_free_time(5, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]), 0);
}
