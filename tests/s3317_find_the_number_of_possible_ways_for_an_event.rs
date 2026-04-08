// Tests for Problem 3317: Find the Number of Possible Ways for an Event
// Java reference: src/test/java/g3301_3400/s3317_find_the_number_of_possible_ways_for_an_event/SolutionTest.java

use leetcode_in_rust::s3317::find_the_number_of_possible_ways_for_an_event::Solution;

#[test]
fn test_number_of_ways() {
    assert_eq!(Solution::number_of_ways(1, 2, 3), 6);
}

#[test]
fn test_number_of_ways2() {
    assert_eq!(Solution::number_of_ways(5, 2, 1), 32);
}

#[test]
fn test_number_of_ways3() {
    assert_eq!(Solution::number_of_ways(3, 3, 4), 684);
}
