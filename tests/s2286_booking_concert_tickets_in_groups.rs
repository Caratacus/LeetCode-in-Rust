// Tests for Problem 2286: Booking Concert Tickets in Groups
// Java reference: src/test/java/g2201_2300/s2286_booking_concert_tickets_in_groups/BookMyShowTest.java

use leetcode_in_rust::s2286::booking_concert_tickets_in_groups::BookMyShow;

#[test]
fn test_book_my_show() {
    // There are 2 rows with 5 seats each
    let mut bms = BookMyShow::new(2, 5);
    // return [0, 0]
    assert_eq!(bms.gather(4, 0), vec![0, 0]);
    // The group books seats [0, 3] of row 0.
    // return []
    assert_eq!(bms.gather(2, 0), vec![]);
    // There is only 1 seat left in row 0,
    // so it is not possible to book 2 consecutive seats.
    // return True
    assert_eq!(bms.scatter(5, 1), true);
    // The group books seat 4 of row 0 and seats [0, 3] of row 1.
    // return False
    assert_eq!(bms.scatter(5, 1), false);
}

#[test]
fn test_book_my_show2() {
    // There are 2 rows with 6 seats each
    let mut bms = BookMyShow::new(2, 6);
    assert_eq!(bms.scatter(2, 1), true);
    assert_eq!(bms.scatter(8, 0), false);
}
