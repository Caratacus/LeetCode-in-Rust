// Tests for Problem 1528: Shuffle String
// Java reference: src/test/java/g1501_1600/s1528_shuffle_string/SolutionTest.java

use leetcode_in_rust::s1528::shuffle_string::Solution;

#[test]
fn test_restore_string() {
    assert_eq!(
        Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode"
    );
}

#[test]
fn test_restore_string2() {
    assert_eq!(
        Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
        "abc"
    );
}
