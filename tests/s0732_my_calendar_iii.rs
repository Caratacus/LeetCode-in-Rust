// Tests for Problem 0732: My Calendar III
// Java reference: src/test/java/g0701_0800/s0732_my_calendar_iii/MyCalendarThreeTest.java

use leetcode_in_rust::s0732::my_calendar_iii::MyCalendarThree;

#[test]
fn test_my_calendar_three() {
    let mut calendar = MyCalendarThree::new();
    assert_eq!(calendar.book(10, 20), 1);
    assert_eq!(calendar.book(50, 60), 1);
    assert_eq!(calendar.book(10, 40), 2);
    assert_eq!(calendar.book(5, 15), 3);
    assert_eq!(calendar.book(5, 10), 3);
    assert_eq!(calendar.book(25, 55), 3);
}
