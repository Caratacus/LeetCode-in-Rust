// Tests for Problem 3330: Find the Original Typed String I
// Java reference: src/test/java/g3301_3400/s3330_find_the_original_typed_string_i/SolutionTest.java

use leetcode_in_rust::s3330::find_the_original_typed_string_i::Solution;

#[test]
fn test_possible_string_count() {
    assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
}

#[test]
fn test_possible_string_count2() {
    assert_eq!(Solution::possible_string_count("abcd".to_string()), 1);
}

#[test]
fn test_possible_string_count3() {
    assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
}
