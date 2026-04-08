// Tests for Problem 3333: Find the Original Typed String II
// Java reference: src/test/java/g3301_3400/s3333_find_the_original_typed_string_ii/SolutionTest.java

use leetcode_in_rust::s3333::find_the_original_typed_string_ii::Solution;

#[test]
fn test_possible_string_count() {
    assert_eq!(Solution::possible_string_count("aabbccdd".to_string(), 7), 5);
}

#[test]
fn test_possible_string_count2() {
    assert_eq!(Solution::possible_string_count("aabbccdd".to_string(), 8), 1);
}

#[test]
fn test_possible_string_count3() {
    assert_eq!(Solution::possible_string_count("aaabbb".to_string(), 3), 8);
}
