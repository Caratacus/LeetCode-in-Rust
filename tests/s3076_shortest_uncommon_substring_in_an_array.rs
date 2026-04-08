// Tests for Problem 3076: Shortest Uncommon Substring in an Array
// Java reference: src/test/java/g3001_3100/s3076_shortest_uncommon_substring_in_an_array/SolutionTest.java

use leetcode_in_rust::s3076::shortest_uncommon_substring_in_an_array::Solution;

#[test]
fn test_shortest_substrings() {
    assert_eq!(
        Solution::shortest_substrings(vec!["cab".to_string(), "ad".to_string(), "bad".to_string(), "c".to_string()]),
        vec!["ab".to_string(), "".to_string(), "ba".to_string(), "".to_string()]
    );
}

#[test]
fn test_shortest_substrings2() {
    assert_eq!(
        Solution::shortest_substrings(vec!["abc".to_string(), "bcd".to_string(), "abcd".to_string()]),
        vec!["".to_string(), "".to_string(), "abcd".to_string()]
    );
}
