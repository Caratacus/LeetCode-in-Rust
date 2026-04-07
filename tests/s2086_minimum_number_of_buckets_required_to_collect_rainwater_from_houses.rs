// Tests for Problem 2086: Minimum Number of Buckets Required to Collect Rainwater from Houses
// Java reference: src/test/java/g2001_2100/s2086_minimum_number_of_buckets_required_to_collect_rainwater_from_houses/SolutionTest.java

use leetcode_in_rust::s2086::minimum_number_of_buckets_required_to_collect_rainwater_from_houses::Solution;

#[test]
fn test_minimum_buckets() {
    assert_eq!(Solution::minimum_buckets("H..H".to_string()), 2);
}

#[test]
fn test_minimum_buckets2() {
    assert_eq!(Solution::minimum_buckets(".H.H.".to_string()), 1);
}

#[test]
fn test_minimum_buckets3() {
    assert_eq!(Solution::minimum_buckets(".HHH.".to_string()), -1);
}
