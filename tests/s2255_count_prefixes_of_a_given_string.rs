// Tests for Problem 2255: Count Prefixes of a Given String
// Java reference: src/test/java/g2201_2300/s2255_count_prefixes_of_a_given_string/SolutionTest.java

use leetcode_in_rust::s2255::count_prefixes_of_a_given_string::Solution;

#[test]
fn test_count_prefixes() {
    assert_eq!(
        Solution::count_prefixes(
            vec!["a".to_string(), "b".to_string(), "c".to_string(), "ab".to_string(), "bc".to_string(), "abc".to_string()],
            "abc".to_string()
        ),
        3
    );
}

#[test]
fn test_count_prefixes2() {
    assert_eq!(
        Solution::count_prefixes(vec!["a".to_string(), "a".to_string()], "aa".to_string()),
        2
    );
}
