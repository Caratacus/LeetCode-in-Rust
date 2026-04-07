// Tests for Problem 2402: Meeting Rooms III
// Java reference: src/test/java/g2401_2500/s2402_meeting_rooms_iii/SolutionTest.java

use leetcode_in_rust::s2402::meeting_rooms_iii::Solution;

#[test]
fn test_most_booked() {
    assert_eq!(
        Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]),
        0
    );
}

#[test]
fn test_most_booked2() {
    assert_eq!(
        Solution::most_booked(3, vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]),
        1
    );
}
