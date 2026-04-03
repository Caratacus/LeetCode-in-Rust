// Tests for Problem 1397: Find All Good Strings
// Java reference: src/test/java/g1301_1400/s1397_find_all_good_strings/SolutionTest.java

use leetcode_in_rust::s1397::find_all_good_strings::Solution;

#[test]
fn test_find_good_strings() {
    assert_eq!(
        Solution::find_good_strings(2, "aa".to_string(), "da".to_string(), "b".to_string()),
        51
    );
}

#[test]
fn test_find_good_strings2() {
    assert_eq!(
        Solution::find_good_strings(
            8,
            "leetcode".to_string(),
            "leetgoes".to_string(),
            "leet".to_string()
        ),
        0
    );
}

#[test]
fn test_find_good_strings3() {
    assert_eq!(
        Solution::find_good_strings(2, "gx".to_string(), "gz".to_string(), "x".to_string()),
        2
    );
}
