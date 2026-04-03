// Tests for Problem 1717: Maximum Score From Removing Substrings
// Java reference: src/test/java/g1701_1800/s1717_maximum_score_from_removing_substrings/SolutionTest.java

use leetcode_in_rust::s1717::maximum_score_from_removing_substrings::Solution;

#[test]
fn test_maximum_gain() {
    assert_eq!(Solution::maximum_gain("cdbcbbaaabab".to_string(), 4, 5), 19);
}

#[test]
fn test_maximum_gain2() {
    assert_eq!(Solution::maximum_gain("aabbaaxybbaabb".to_string(), 5, 4), 20);
}
