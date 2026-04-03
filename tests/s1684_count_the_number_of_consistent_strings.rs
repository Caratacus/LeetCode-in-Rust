// Tests for Problem 1684: Count the Number of Consistent Strings
// Java reference: src/test/java/g1601_1700/s1684_count_the_number_of_consistent_strings/SolutionTest.java

use leetcode_in_rust::s1684::count_the_number_of_consistent_strings::Solution;

#[test]
fn test_count_consistent_strings() {
    assert_eq!(
        Solution::count_consistent_strings(
            "ab".to_string(),
            vec!["ad".to_string(), "bd".to_string(), "aaab".to_string(), "baa".to_string(), "badab".to_string()]
        ),
        2
    );
}

#[test]
fn test_count_consistent_strings2() {
    assert_eq!(
        Solution::count_consistent_strings(
            "abc".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string(), "ab".to_string(), "ac".to_string(), "bc".to_string(), "abc".to_string()]
        ),
        7
    );
}

#[test]
fn test_count_consistent_strings3() {
    assert_eq!(
        Solution::count_consistent_strings(
            "cad".to_string(),
            vec!["cc".to_string(), "acd".to_string(), "b".to_string(), "ba".to_string(), "bac".to_string(), "bad".to_string(), "ac".to_string(), "d".to_string()]
        ),
        4
    );
}
