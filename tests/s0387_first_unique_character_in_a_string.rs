// Tests for Problem 0387: First Unique Character in a String
// Java reference: src/test/java/g0301_0400/s0387_first_unique_character_in_a_string/SolutionTest.java

use leetcode_in_rust::s0387::first_unique_character_in_a_string::Solution;

#[test]
fn test_first_uniq_char() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
}

#[test]
fn test_first_uniq_char2() {
    assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
}

#[test]
fn test_first_uniq_char3() {
    assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
}
