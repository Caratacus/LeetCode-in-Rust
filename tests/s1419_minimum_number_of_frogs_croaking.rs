// Tests for Problem 1419: Minimum Number of Frogs Croaking
// Java reference: src/test/java/g1401_1500/s1419_minimum_number_of_frogs_croaking/SolutionTest.java

use leetcode_in_rust::s1419::minimum_number_of_frogs_croaking::Solution;

#[test]
fn test_min_number_of_frogs() {
    assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()), 1);
}

#[test]
fn test_min_number_of_frogs2() {
    assert_eq!(Solution::min_number_of_frogs("crcoakroak".to_string()), 2);
}

#[test]
fn test_min_number_of_frogs3() {
    assert_eq!(Solution::min_number_of_frogs("croakcrook".to_string()), -1);
}
