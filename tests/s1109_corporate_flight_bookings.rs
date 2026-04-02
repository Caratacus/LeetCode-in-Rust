// Tests for Problem 1109: Corporate Flight Bookings
// Java reference: src/test/java/g1101_1200/s1109_corporate_flight_bookings/SolutionTest.java

use leetcode_in_rust::s1109::corporate_flight_bookings::Solution;

#[test]
fn test_corp_flight_bookings() {
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
        vec![10, 55, 45, 25, 25]
    );
}

#[test]
fn test_corp_flight_bookings2() {
    assert_eq!(
        Solution::corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 2, 15]], 2),
        vec![10, 25]
    );
}
