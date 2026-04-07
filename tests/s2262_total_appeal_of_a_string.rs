// Tests for Problem 2262: Total Appeal of a String
// Java reference: src/test/java/g2201_2300/s2262_total_appeal_of_a_string/SolutionTest.java

use leetcode_in_rust::s2262::total_appeal_of_a_string::Solution;

#[test]
fn test_appeal_sum() {
    assert_eq!(Solution::appeal_sum("abbca".to_string()), 28);
}

#[test]
fn test_appeal_sum2() {
    assert_eq!(Solution::appeal_sum("code".to_string()), 20);
}
