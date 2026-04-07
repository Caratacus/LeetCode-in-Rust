// Tests for Problem 2851: String Transformation
// Java reference: src/test/java/g2801_2900/s2851_string_transformation/SolutionTest.java

use leetcode_in_rust::s2851::string_transformation::Solution;

#[test]
fn test_number_of_ways() {
    assert_eq!(Solution::number_of_ways("abcd".to_string(), "cdab".to_string(), 2), 2);
}

#[test]
fn test_number_of_ways2() {
    assert_eq!(Solution::number_of_ways("ababab".to_string(), "ababab".to_string(), 1), 2);
}
