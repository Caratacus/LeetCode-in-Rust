// Tests for Problem 0729: My Calendar I
// Java reference: src/test/java/g0701_0800/s0729_my_calendar_i/MyCalendarTest.java

use leetcode_in_rust::s0729::my_calendar_i::MyCalendar;

#[test]
fn test_my_calendar() {
    let mut calendar = MyCalendar::new();
    assert_eq!(calendar.book(10, 20), true);
    assert_eq!(calendar.book(15, 25), false);
    assert_eq!(calendar.book(20, 30), true);
}
