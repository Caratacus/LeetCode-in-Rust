// Tests for Problem 1227: Airplane Seat Assignment Probability
// Java reference: src/test/java/g1201_1300/s1227_airplane_seat_assignment_probability/SolutionTest.java

use leetcode_in_rust::s1227::airplane_seat_assignment_probability::Solution;

#[test]
fn test_nth_person_gets_nth_seat() {
    let result = Solution::nth_person_gets_nth_seat(1);
    assert!((result - 1.0).abs() < 1e-5);
}

#[test]
fn test_nth_person_gets_nth_seat2() {
    let result = Solution::nth_person_gets_nth_seat(2);
    assert!((result - 0.5).abs() < 1e-5);
}
