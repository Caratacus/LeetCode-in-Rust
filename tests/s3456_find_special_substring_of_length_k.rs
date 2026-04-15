// Tests for Problem 3456: Find Special Substring of Length K
// Java reference: src/test/java/g3401_3500/s3456_find_special_substring_of_length_k/SolutionTest.java

use leetcode_in_rust::s3456::find_special_substring_of_length_k::Solution;

#[test]
fn test_has_special_substring() {
    assert_eq!(Solution::has_special_substring("aaabaaa".to_string(), 3), true);
}

#[test]
fn test_has_special_substring2() {
    assert_eq!(Solution::has_special_substring("abc".to_string(), 2), false);
}

#[test]
fn test_has_special_substring3() {
    assert_eq!(Solution::has_special_substring("ccc".to_string(), 2), false);
}
