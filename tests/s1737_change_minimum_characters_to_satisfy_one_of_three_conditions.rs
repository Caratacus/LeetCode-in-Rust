// Tests for Problem 1737: Change Minimum Characters to Satisfy One of Three Conditions
// Java reference: src/test/java/g1701_1800/s1737_change_minimum_characters_to_satisfy_one_of_three_conditions/SolutionTest.java

use leetcode_in_rust::s1737::change_minimum_characters_to_satisfy_one_of_three_conditions::Solution;

#[test]
fn test_min_characters() {
    assert_eq!(Solution::min_characters("aba".to_string(), "caa".to_string()), 2);
}

#[test]
fn test_min_characters2() {
    assert_eq!(Solution::min_characters("dabadd".to_string(), "cda".to_string()), 3);
}
