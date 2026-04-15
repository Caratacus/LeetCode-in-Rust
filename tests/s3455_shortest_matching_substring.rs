// Tests for Problem 3455: Shortest Matching Substring
// Java reference: src/test/java/g3401_3500/s3455_shortest_matching_substring/SolutionTest.java

use leetcode_in_rust::s3455::shortest_matching_substring::Solution;

#[test]
fn test_shortest_matching_substring() {
    assert_eq!(Solution::shortest_matching_substring("abaacbaecebce".to_string(), "ba*c*ce".to_string()), 8);
}

#[test]
fn test_shortest_matching_substring2() {
    assert_eq!(Solution::shortest_matching_substring("baccbaadbc".to_string(), "cc*baa*adb".to_string()), -1);
}

#[test]
fn test_shortest_matching_substring3() {
    assert_eq!(Solution::shortest_matching_substring("a".to_string(), "**".to_string()), 0);
}

#[test]
fn test_shortest_matching_substring4() {
    assert_eq!(Solution::shortest_matching_substring("madlogic".to_string(), "*adlogi*".to_string()), 6);
}
