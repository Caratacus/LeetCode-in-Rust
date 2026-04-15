// Tests for Problem 3498: Reverse Degree of a String
// Java reference: src/test/java/g3401_3500/s3498_reverse_degree_of_a_string/SolutionTest.java

use leetcode_in_rust::s3498::reverse_degree_of_a_string::Solution;

#[test]
fn test_reverse_degree() {
    assert_eq!(Solution::reverse_degree("abc".to_string()), 148);
}

#[test]
fn test_reverse_degree2() {
    assert_eq!(Solution::reverse_degree("zaza".to_string()), 160);
}
