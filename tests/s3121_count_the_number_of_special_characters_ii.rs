// Tests for Problem 3121: Count the Number of Special Characters II
// Java reference: src/test/java/g3101_3200/s3121_count_the_number_of_special_characters_ii/SolutionTest.java

use leetcode_in_rust::s3121::count_the_number_of_special_characters_ii::Solution;

#[test]
fn test_number_of_special_chars() {
    assert_eq!(Solution::number_of_special_chars("aaAbcBC".to_string()), 3);
}

#[test]
fn test_number_of_special_chars2() {
    assert_eq!(Solution::number_of_special_chars("abc".to_string()), 0);
}

#[test]
fn test_number_of_special_chars3() {
    assert_eq!(Solution::number_of_special_chars("AbBCab".to_string()), 0);
}
