// Tests for Problem 3709: Design Exam Scores Tracker
// Java reference: src/test/java/g3701_3800/s3709_design_exam_scores_tracker/ExamTrackerTest.java
use leetcode_in_rust::s3709::design_exam_scores_tracker::ExamTracker;
#[test]
fn test_exam_tracker() {
    let mut exam_tracker = ExamTracker::new();
    exam_tracker.record(1, 98);
    assert_eq!(exam_tracker.total_score(1, 1), 98i64);
    exam_tracker.record(5, 99);
    assert_eq!(exam_tracker.total_score(1, 3), 98i64);
    assert_eq!(exam_tracker.total_score(1, 5), 197i64);
    assert_eq!(exam_tracker.total_score(3, 4), 0i64);
    assert_eq!(exam_tracker.total_score(2, 5), 99i64);
}
