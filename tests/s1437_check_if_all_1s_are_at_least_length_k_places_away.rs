// Tests for Problem 1437: Check If All 1's Are at Least Length K Places Away
// Java reference: src/test/java/g1401_1500/s1437_check_if_all_1s_are_at_least_length_k_places_away/SolutionTest.java

use leetcode_in_rust::s1437::check_if_all_1s_are_at_least_length_k_places_away::Solution;

#[test]
fn test_k_length_apart() {
    assert_eq!(Solution::k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
}

#[test]
fn test_k_length_apart2() {
    assert_eq!(Solution::k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
}
