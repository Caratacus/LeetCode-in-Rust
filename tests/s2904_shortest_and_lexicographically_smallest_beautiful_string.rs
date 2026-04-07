// Tests for Problem 2904: Shortest and Lexicographically Smallest Beautiful String
// Java reference: src/test/java/g2901_3000/s2904_shortest_and_lexicographically_smallest_beautiful_string/SolutionTest.java

use leetcode_in_rust::s2904::shortest_and_lexicographically_smallest_beautiful_string::Solution;

#[test]
fn test_shortest_beautiful_substring() {
    assert_eq!(Solution::shortest_beautiful_substring("100011001".to_string(), 3), "11001");
}

#[test]
fn test_shortest_beautiful_substring2() {
    assert_eq!(Solution::shortest_beautiful_substring("1011".to_string(), 2), "11");
}

#[test]
fn test_shortest_beautiful_substring3() {
    assert_eq!(Solution::shortest_beautiful_substring("000".to_string(), 1), "");
}
