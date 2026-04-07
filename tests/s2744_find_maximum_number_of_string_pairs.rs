// Tests for Problem 2744: Find Maximum Number of String Pairs
// Java reference: src/test/java/g2701_2800/s2744_find_maximum_number_of_string_pairs/SolutionTest.java

use leetcode_in_rust::s2744::find_maximum_number_of_string_pairs::Solution;

#[test]
fn test_maximum_number_of_string_pairs() {
    assert_eq!(
        Solution::maximum_number_of_string_pairs(vec![
            "cd".to_string(),
            "ac".to_string(),
            "dc".to_string(),
            "ca".to_string(),
            "zz".to_string()
        ]),
        2
    );
}

#[test]
fn test_maximum_number_of_string_pairs2() {
    assert_eq!(
        Solution::maximum_number_of_string_pairs(vec!["ab".to_string(), "ba".to_string(), "cc".to_string()]),
        1
    );
}

#[test]
fn test_maximum_number_of_string_pairs3() {
    assert_eq!(
        Solution::maximum_number_of_string_pairs(vec!["aa".to_string(), "ab".to_string()]),
        0
    );
}
