// Tests for Problem 0551: Student Attendance Record I
// Java reference: src/test/java/g0501_0600/s0551_student_attendance_record_i/SolutionTest.java

use leetcode_in_rust::s0551::student_attendance_record_i::Solution;

#[test]
fn test_check_record() {
    assert_eq!(Solution::check_record("PPALLP".to_string()), true);
}

#[test]
fn test_check_record2() {
    assert_eq!(Solution::check_record("PPALLL".to_string()), false);
}

#[test]
fn test_check_record3() {
    assert_eq!(Solution::check_record("ALLAPPL".to_string()), false);
}
