// Tests for Problem 3168: Minimum Number of Chairs in a Waiting Room
// Java reference: src/test/java/g3101_3200/s3168_minimum_number_of_chairs_in_a_waiting_room/SolutionTest.java

use leetcode_in_rust::s3168::minimum_number_of_chairs_in_a_waiting_room::Solution;

#[test]
fn test_minimum_chairs() {
    assert_eq!(Solution::minimum_chairs(String::from("EEEEEEE")), 7);
}
#[test]
fn test_minimum_chairs2() {
    assert_eq!(Solution::minimum_chairs(String::from("ELELEEL")), 2);
}
#[test]
fn test_minimum_chairs3() {
    assert_eq!(Solution::minimum_chairs(String::from("ELEELEELLL")), 3);
}