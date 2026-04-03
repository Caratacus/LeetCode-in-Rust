// Tests for Problem 1520: Maximum Number of Non-Overlapping Substrings
// Java reference: src/test/java/g1501_1600/s1520_maximum_number_of_non_overlapping_substrings/SolutionTest.java

use leetcode_in_rust::s1520::maximum_number_of_non_overlapping_substrings::Solution;

#[test]
fn test_max_num_of_substrings() {
    let mut result = Solution::max_num_of_substrings("adefaddaccc".to_string());
    result.sort();
    assert_eq!(result, vec!["ccc".to_string(), "e".to_string(), "f".to_string()]);
}

#[test]
fn test_max_num_of_substrings2() {
    let mut result = Solution::max_num_of_substrings("abbaccd".to_string());
    result.sort();
    assert_eq!(result, vec!["bb".to_string(), "cc".to_string(), "d".to_string()]);
}

#[test]
fn test_max_num_of_substrings3() {
    assert_eq!(Solution::max_num_of_substrings("a".to_string()), vec!["a".to_string()]);
}

#[test]
fn test_max_num_of_substrings4() {
    let mut result = Solution::max_num_of_substrings("abc".to_string());
    result.sort();
    assert_eq!(result, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
fn test_max_num_of_substrings5() {
    let mut result = Solution::max_num_of_substrings("abac".to_string());
    result.sort();
    assert_eq!(result, vec!["b".to_string(), "c".to_string()]);
}

#[test]
fn test_max_num_of_substrings6() {
    let mut result = Solution::max_num_of_substrings("bba".to_string());
    result.sort();
    assert_eq!(result, vec!["a".to_string(), "bb".to_string()]);
}

#[test]
fn test_max_num_of_substrings7() {
    assert_eq!(Solution::max_num_of_substrings("abcabc".to_string()), vec!["abcabc".to_string()]);
}

#[test]
fn test_max_num_of_substrings8() {
    assert_eq!(Solution::max_num_of_substrings("aaaa".to_string()), vec!["aaaa".to_string()]);
}

#[test]
fn test_max_num_of_substrings9() {
    assert_eq!(Solution::max_num_of_substrings("".to_string()), Vec::<String>::new());
}

#[test]
fn test_max_num_of_substrings10() {
    assert_eq!(Solution::max_num_of_substrings("cabcccbaa".to_string()), vec!["cabcccbaa".to_string()]);
}
