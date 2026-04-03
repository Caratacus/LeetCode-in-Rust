// Tests for Problem 1750: Minimum Length of String After Deleting Similar Ends
// Java reference: src/test/java/g1701_1800/s1750_minimum_length_of_string_after_deleting_similar_ends/SolutionTest.java

use leetcode_in_rust::s1750::minimum_length_of_string_after_deleting_similar_ends::Solution;

#[test]
fn test_minimum_length() {
    assert_eq!(Solution::minimum_length("ca".to_string()), 2);
}

#[test]
fn test_minimum_length2() {
    assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
}

#[test]
fn test_minimum_length3() {
    assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
}
