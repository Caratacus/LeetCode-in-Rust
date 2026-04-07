// Tests for Problem 1974: Minimum Time to Type Word Using Special Typewriter
// Java reference: src/test/java/g1901_2000/s1974_minimum_time_to_type_word_using_special_typewriter/SolutionTest.java

use leetcode_in_rust::s1974::minimum_time_to_type_word_using_special_typewriter::Solution;

#[test]
fn test_min_time_to_type() {
    assert_eq!(Solution::min_time_to_type(String::from("abc")), 5);
}

#[test]
fn test_min_time_to_type2() {
    assert_eq!(Solution::min_time_to_type(String::from("bza")), 7);
}

#[test]
fn test_min_time_to_type3() {
    assert_eq!(Solution::min_time_to_type(String::from("zjpc")), 34);
}
