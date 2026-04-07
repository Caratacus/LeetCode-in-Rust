// Tests for Problem 2037: Minimum Number of Moves to Seat Everyone
// Java reference: src/test/java/g2001_2100/s2037_minimum_number_of_moves_to_seat_everyone/SolutionTest.java

use leetcode_in_rust::s2037::minimum_number_of_moves_to_seat_everyone::Solution;

#[test]
fn test_min_moves_to_seat() {
    assert_eq!(Solution::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
}

#[test]
fn test_min_moves_to_seat2() {
    assert_eq!(
        Solution::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]),
        7
    );
}

#[test]
fn test_min_moves_to_seat3() {
    assert_eq!(
        Solution::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]),
        4
    );
}
