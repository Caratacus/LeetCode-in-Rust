// Tests for Problem 2053: Kth Distinct String in an Array
// Java reference: src/test/java/g2001_2100/s2053_kth_distinct_string_in_an_array/SolutionTest.java

use leetcode_in_rust::s2053::kth_distinct_string_in_an_array::Solution;

#[test]
fn test_kth_distinct() {
    assert_eq!(
        Solution::kth_distinct(
            vec!["d".to_string(), "b".to_string(), "c".to_string(), "b".to_string(), "c".to_string(), "a".to_string()],
            2
        ),
        "a"
    );
}

#[test]
fn test_kth_distinct2() {
    assert_eq!(
        Solution::kth_distinct(vec!["aaa".to_string(), "aa".to_string(), "a".to_string()], 1),
        "aaa"
    );
}

#[test]
fn test_kth_distinct3() {
    assert_eq!(
        Solution::kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3),
        ""
    );
}
