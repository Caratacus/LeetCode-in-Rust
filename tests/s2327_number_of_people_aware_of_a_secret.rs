// Tests for Problem 2327: Number of People Aware of a Secret
// Java reference: src/test/java/g2301_2400/s2327_number_of_people_aware_of_a_secret/SolutionTest.java

use leetcode_in_rust::s2327::number_of_people_aware_of_a_secret::Solution;

#[test]
fn test_people_aware_of_secret() {
    assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
}

#[test]
fn test_people_aware_of_secret2() {
    assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
}
