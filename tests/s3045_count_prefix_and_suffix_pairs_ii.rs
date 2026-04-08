// Tests for Problem 3045: Count Prefix and Suffix Pairs II
// Java reference: src/test/java/g3001_3100/s3045_count_prefix_and_suffix_pairs_ii/SolutionTest.java

use leetcode_in_rust::s3045::count_prefix_and_suffix_pairs_ii::Solution;

#[test]
fn test_count_prefix_suffix_pairs() {
    let words = vec![
        String::from("a"),
        String::from("aba"),
        String::from("ababa"),
        String::from("aa"),
    ];
    assert_eq!(Solution::count_prefix_suffix_pairs(words), 4);
}

#[test]
fn test_count_prefix_suffix_pairs2() {
    let words = vec![
        String::from("pa"),
        String::from("papa"),
        String::from("ma"),
        String::from("mama"),
    ];
    assert_eq!(Solution::count_prefix_suffix_pairs(words), 2);
}

#[test]
fn test_count_prefix_suffix_pairs3() {
    let words = vec![String::from("abab"), String::from("ab")];
    assert_eq!(Solution::count_prefix_suffix_pairs(words), 0);
}
