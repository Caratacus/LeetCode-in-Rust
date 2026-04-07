// Tests for Problem 2506: Count Pairs Of Similar Strings
// Java reference: src/test/java/g2401_2500/s2506_count_pairs_of_similar_strings/SolutionTest.java

use leetcode_in_rust::s2506::count_pairs_of_similar_strings::Solution;

#[test]
fn test_similar_pairs() {
    assert_eq!(Solution::similar_pairs(vec!["aba".to_string(), "aabb".to_string(), "abcd".to_string(), "bac".to_string(), "aabc".to_string()]), 2);
}

#[test]
fn test_similar_pairs2() {
    assert_eq!(Solution::similar_pairs(vec!["aabb".to_string(), "ab".to_string(), "abc".to_string()]), 3);
}

#[test]
fn test_similar_pairs3() {
    assert_eq!(Solution::similar_pairs(vec!["nba".to_string(), "cba".to_string(), "dba".to_string()]), 0);
}
