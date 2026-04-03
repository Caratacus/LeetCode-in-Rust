// Tests for Problem 1507: Reformat Date
// Java reference: src/test/java/g1501_1600/s1507_reformat_date/SolutionTest.java

use leetcode_in_rust::s1507::reformat_date::Solution;

#[test]
fn test_reformat_date() {
    assert_eq!(Solution::reformat_date("20th Oct 2052".to_string()), "2052-10-20");
}

#[test]
fn test_reformat_date2() {
    assert_eq!(Solution::reformat_date("6th Jun 1933".to_string()), "1933-06-06");
}

#[test]
fn test_reformat_date3() {
    assert_eq!(Solution::reformat_date("26th May 1960".to_string()), "1960-05-26");
}
