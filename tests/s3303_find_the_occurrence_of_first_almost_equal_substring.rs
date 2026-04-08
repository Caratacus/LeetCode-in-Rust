// Tests for Problem 3303: Find the Occurrence of First Almost Equal Substring
// Java reference: src/test/java/g3301_3400/s3303_find_the_occurrence_of_first_almost_equal_substring/SolutionTest.java

use leetcode_in_rust::s3303::find_the_occurrence_of_first_almost_equal_substring::Solution;

#[test]
fn test_min_starting_index() {
    assert_eq!(Solution::min_starting_index("abcdefg".to_string(), "bcdffg".to_string()), 1);
}

#[test]
fn test_min_starting_index2() {
    assert_eq!(Solution::min_starting_index("ababbababa".to_string(), "bacaba".to_string()), 4);
}

#[test]
fn test_min_starting_index3() {
    assert_eq!(Solution::min_starting_index("abcd".to_string(), "dba".to_string()), -1);
}

#[test]
fn test_min_starting_index4() {
    assert_eq!(Solution::min_starting_index("dde".to_string(), "d".to_string()), 0);
}
