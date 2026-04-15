// Tests for Problem 3458: Select K Disjoint Special Substrings
// Java reference: src/test/java/g3401_3500/s3458_select_k_disjoint_special_substrings/SolutionTest.java

use leetcode_in_rust::s3458::select_k_disjoint_special_substrings::Solution;

#[test]
fn test_max_substring_length() {
    assert_eq!(Solution::max_substring_length("abcdbaefab".to_string(), 2), true);
}

#[test]
fn test_max_substring_length2() {
    assert_eq!(Solution::max_substring_length("cdefdc".to_string(), 3), false);
}

#[test]
fn test_max_substring_length3() {
    assert_eq!(Solution::max_substring_length("abeabe".to_string(), 0), true);
}
