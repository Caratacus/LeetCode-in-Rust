// Tests for Problem 3439: Reschedule Meetings for Maximum Free Time I
// Java reference: src/test/java/g3401_3500/s3439_reschedule_meetings_for_maximum_free_time_i/SolutionTest.java

use leetcode_in_rust::s3439::reschedule_meetings_for_maximum_free_time_i::Solution;

#[test]
fn test_max_free_time() {
    assert_eq!(Solution::max_free_time(5, 1, vec![1, 3], vec![2, 5]), 2);
}

#[test]
fn test_max_free_time2() {
    assert_eq!(Solution::max_free_time(10, 1, vec![0, 2, 9], vec![1, 4, 10]), 6);
}

#[test]
fn test_max_free_time3() {
    assert_eq!(
        Solution::max_free_time(5, 2, vec![0, 1, 2, 3, 4], vec![1, 2, 3, 4, 5]),
        0
    );
}
