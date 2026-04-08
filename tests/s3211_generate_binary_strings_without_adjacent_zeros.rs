// Tests for Problem 3211: Generate Binary Strings Without Adjacent Zeros
// Java reference: src/test/java/g3201_3300/s3211_generate_binary_strings_without_adjacent_zeros/SolutionTest.java

use leetcode_in_rust::s3211::generate_binary_strings_without_adjacent_zeros::Solution;

#[test]
fn test_valid_strings() {
    let mut result = Solution::valid_strings(3);
    let mut expected = vec!["111".to_string(), "110".to_string(), "101".to_string(), "011".to_string(), "010".to_string()];
    result.sort();
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_valid_strings2() {
    let mut result = Solution::valid_strings(1);
    let mut expected = vec!["1".to_string(), "0".to_string()];
    result.sort();
    expected.sort();
    assert_eq!(result, expected);
}
