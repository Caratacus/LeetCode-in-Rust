// Tests for Problem 1845: Seat Reservation Manager
// Java reference: src/test/java/g1801_1900/s1845_seat_reservation_manager/SeatManagerTest.java

use leetcode_in_rust::s1845::seat_reservation_manager::SeatManager;

#[test]
fn test_seat_manager() {
    let mut seat_manager = SeatManager::new(5);
    assert_eq!(seat_manager.reserve(), 1);
    assert_eq!(seat_manager.reserve(), 2);
    seat_manager.unreserve(2);
    assert_eq!(seat_manager.reserve(), 2);
    assert_eq!(seat_manager.reserve(), 3);
    assert_eq!(seat_manager.reserve(), 4);
    assert_eq!(seat_manager.reserve(), 5);
    seat_manager.unreserve(5);
}
