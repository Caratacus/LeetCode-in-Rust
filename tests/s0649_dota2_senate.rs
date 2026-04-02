// Tests for Problem 0649: Dota2 Senate
// Java reference: src/test/java/g0601_0700/s0649_dota2_senate/SolutionTest.java

use leetcode_in_rust::s0649::dota2_senate::Solution;

#[test]
fn test_predict_party_victory() {
    assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
}

#[test]
fn test_predict_party_victory2() {
    assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
}
