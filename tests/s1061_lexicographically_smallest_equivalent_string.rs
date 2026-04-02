// Tests for Problem 1061: Lexicographically Smallest Equivalent String
// Java reference: src/test/java/g1001_1100/s1061_lexicographically_smallest_equivalent_string/SolutionTest.java

use leetcode_in_rust::s1061::lexicographically_smallest_equivalent_string::Solution;

#[test]
fn test_smallest_equivalent_string() {
    assert_eq!(
        Solution::smallest_equivalent_string("hello".to_string(), "world".to_string(), "hold".to_string()),
        "hdld"
    );
}

#[test]
fn test_smallest_equivalent_string2() {
    assert_eq!(
        Solution::smallest_equivalent_string("parker".to_string(), "morris".to_string(), "parser".to_string()),
        "makkek"
    );
}

#[test]
fn test_smallest_equivalent_string3() {
    assert_eq!(
        Solution::smallest_equivalent_string("leetcode".to_string(), "programs".to_string(), "sourcecode".to_string()),
        "aauaaaaada"
    );
}
