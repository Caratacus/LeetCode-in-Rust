// Tests for Problem 3120: Count the Number of Special Characters I
// Java reference: src/test/java/g3101_3200/s3120_count_the_number_of_special_characters_i/SolutionTest.java

use leetcode_in_rust::s3120::count_the_number_of_special_characters_i::Solution;

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
    assert_eq!(Solution::number_of_special_chars("abBCab".to_string()), 1);
}
