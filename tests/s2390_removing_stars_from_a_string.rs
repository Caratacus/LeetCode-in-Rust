// Tests for Problem 2390: Removing Stars From a String
// Java reference: src/test/java/g2301_2400/s2390_removing_stars_from_a_string/SolutionTest.java

use leetcode_in_rust::s2390::removing_stars_from_a_string::Solution;

#[test]
fn test_remove_stars() {
    assert_eq!(Solution::remove_stars("leet**cod*e".to_string()), "lecoe");
}

#[test]
fn test_remove_stars2() {
    assert_eq!(Solution::remove_stars("erase*****".to_string()), "");
}
