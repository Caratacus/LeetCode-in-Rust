// Tests for Problem 3016: Minimum Number of Pushes to Type Word II
// Java reference: src/test/java/g3001_3100/s3016_minimum_number_of_pushes_to_type_word_ii/SolutionTest.java

use leetcode_in_rust::s3016::minimum_number_of_pushes_to_type_word_ii::Solution;

#[test]
fn test_minimum_pushes() {
    assert_eq!(Solution::minimum_pushes(String::from("abcde")), 5);
}

#[test]
fn test_minimum_pushes2() {
    assert_eq!(Solution::minimum_pushes(String::from("xyzxyzxyzxyz")), 12);
}

#[test]
fn test_minimum_pushes3() {
    assert_eq!(Solution::minimum_pushes(String::from("aabbccddeeffgghhiiiiii")), 24);
}
