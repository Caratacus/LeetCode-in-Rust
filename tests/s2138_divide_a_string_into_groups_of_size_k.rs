// Tests for Problem 2138: Divide a String Into Groups of Size k
// Java reference: src/test/java/g2101_2200/s2138_divide_a_string_into_groups_of_size_k/SolutionTest.java

use leetcode_in_rust::s2138::divide_a_string_into_groups_of_size_k::Solution;

#[test]
fn test_divide_string() {
    assert_eq!(
        Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
        vec!["abc".to_string(), "def".to_string(), "ghi".to_string()]
    );
}

#[test]
fn test_divide_string2() {
    assert_eq!(
        Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
        vec![
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
            "jxx".to_string()
        ]
    );
}
