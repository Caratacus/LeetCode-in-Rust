// Tests for Problem 0731: My Calendar II
// Java reference: src/test/java/g0701_0800/s0731_my_calendar_ii/MyCalendarTwoTest.java

use leetcode_in_rust::s0731::my_calendar_ii::MyCalendarTwo;

#[test]
fn test_my_calendar_two() {
    let mut calendar = MyCalendarTwo::new();
    assert_eq!(calendar.book(10, 20), true);
    assert_eq!(calendar.book(50, 60), true);
    assert_eq!(calendar.book(10, 40), true);
    assert_eq!(calendar.book(5, 15), false);
    assert_eq!(calendar.book(5, 10), true);
    assert_eq!(calendar.book(25, 55), true);
}
