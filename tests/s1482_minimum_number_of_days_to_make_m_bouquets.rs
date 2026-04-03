// Tests for Problem 1482: Minimum Number of Days to Make m Bouquets
// Java reference: src/test/java/g1401_1500/s1482_minimum_number_of_days_to_make_m_bouquets/SolutionTest.java

use leetcode_in_rust::s1482::minimum_number_of_days_to_make_m_bouquets::Solution;

#[test]
fn test_min_days() {
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
}

#[test]
fn test_min_days2() {
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
}

#[test]
fn test_min_days3() {
    assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
}
