// Tests for Problem 3184: Count Pairs That Form a Complete Day I
// Java reference: src/test/java/g3101_3200/s3184_count_pairs_that_form_a_complete_day_i/SolutionTest.java

use leetcode_in_rust::s3184::count_pairs_that_form_a_complete_day_i::Solution;

#[test]
fn test_count_complete_day_pairs() {
    assert_eq!(Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]), 2);
}

#[test]
fn test_count_complete_day_pairs2() {
    assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
}
