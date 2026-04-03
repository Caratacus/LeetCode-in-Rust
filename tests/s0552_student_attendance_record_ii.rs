// Tests for Problem 0552: Student Attendance Record II
// Java reference: src/test/java/g0501_0600/s0552_student_attendance_record_ii/SolutionTest.java

use leetcode_in_rust::s0552::student_attendance_record_ii::Solution;

#[test]
fn test_check_record() {
    assert_eq!(Solution::check_record(2), 8);
}

#[test]
fn test_check_record2() {
    assert_eq!(Solution::check_record(1), 3);
}

#[test]
fn test_check_record3() {
    assert_eq!(Solution::check_record(10101), 183236316);
}
