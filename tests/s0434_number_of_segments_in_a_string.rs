// Tests for Problem 0434: Number of Segments in a String
// Java reference: src/test/java/g0401_0500/s0434_number_of_segments_in_a_string/SolutionTest.java

use leetcode_in_rust::s0434::number_of_segments_in_a_string::Solution;

#[test]
fn test_count_segments() {
    assert_eq!(Solution::count_segments("Hello, my name is John".to_string()), 5);
}

#[test]
fn test_count_segments2() {
    assert_eq!(Solution::count_segments("Hello".to_string()), 1);
}
