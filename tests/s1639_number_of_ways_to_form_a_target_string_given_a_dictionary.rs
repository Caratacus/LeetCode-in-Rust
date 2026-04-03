// Tests for Problem 1639: Number of Ways to Form a Target String Given a Dictionary
// Java reference: src/test/java/g1601_1700/s1639_number_of_ways_to_form_a_target_string_given_a_dictionary/SolutionTest.java

use leetcode_in_rust::s1639::number_of_ways_to_form_a_target_string_given_a_dictionary::Solution;

#[test]
fn test_num_ways() {
    assert_eq!(Solution::num_ways(vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()], "aba".to_string()), 6);
}

#[test]
fn test_num_ways2() {
    assert_eq!(Solution::num_ways(vec!["abba".to_string(), "baab".to_string()], "bab".to_string()), 4);
}
