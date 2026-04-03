// Tests for Problem 0855: Exam Room
// Java reference: src/test/java/g0801_0900/s0855_exam_room/ExamRoomTest.java

use leetcode_in_rust::s0855::exam_room::ExamRoom;

#[test]
fn test_exam_room() {
    let mut exam_room = ExamRoom::new(10);
    assert_eq!(exam_room.seat(), 0);
    assert_eq!(exam_room.seat(), 9);
    assert_eq!(exam_room.seat(), 4);
    assert_eq!(exam_room.seat(), 2);
    exam_room.leave(4);
    assert_eq!(exam_room.seat(), 5);
}
