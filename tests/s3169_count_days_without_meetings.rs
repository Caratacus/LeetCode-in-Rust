// Tests for Problem 3169: Count Days Without Meetings
// Java reference: src/test/java/g3101_3200/s3169_count_days_without_meetings/SolutionTest.java

use leetcode_in_rust::s3169::count_days_without_meetings::Solution;
#[test]
fn test_count_days() {
    assert_eq!(Solution::count_days(10, vec![vec![5, 7], vec![1, 3], vec![9, 10]]), 2);
}
#[test]
fn test_count_days2() {
    assert_eq!(Solution::count_days(5, vec![vec![2, 4], vec![1, 3]]), 1);
}
#[test]
fn test_count_days3() {
    assert_eq!(Solution::count_days(6, vec![vec![1, 6]]), 0);
}
