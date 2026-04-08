// Tests for Problem 3083: Existence of a Substring in a String and Its Reverse
// Java reference: src/test/java/g3001_3100/s3083_existence_of_a_substring_in_a_string_and_its_reverse/SolutionTest.java

use leetcode_in_rust::s3083::existence_of_a_substring_in_a_string_and_its_reverse::Solution;

#[test]
fn test_is_substring_present() {
    assert_eq!(Solution::is_substring_present("leetcode".to_string()), true);
}

#[test]
fn test_is_substring_present2() {
    assert_eq!(Solution::is_substring_present("abcba".to_string()), true);
}

#[test]
fn test_is_substring_present3() {
    assert_eq!(Solution::is_substring_present("abcd".to_string()), false);
}
