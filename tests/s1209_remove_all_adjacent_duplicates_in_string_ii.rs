// Tests for Problem 1209: Remove All Adjacent Duplicates in String II
// Java reference: src/test/java/g1201_1300/s1209_remove_all_adjacent_duplicates_in_string_ii/SolutionTest.java

use leetcode_in_rust::s1209::remove_all_adjacent_duplicates_in_string_ii::Solution;

#[test]
fn test_remove_duplicates() {
    let result = Solution::remove_duplicates("abcd".to_string(), 2);
    assert_eq!(result, "abcd");
}

#[test]
fn test_remove_duplicates2() {
    let result = Solution::remove_duplicates("deeedbbcccbdaa".to_string(), 3);
    assert_eq!(result, "aa");
}

#[test]
fn test_remove_duplicates3() {
    let result = Solution::remove_duplicates("pbbcggttciiippooaais".to_string(), 2);
    assert_eq!(result, "ps");
}
