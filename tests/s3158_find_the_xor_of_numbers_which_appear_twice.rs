// Tests for Problem 3158: Find the XOR of Numbers Which Appear Twice
// Java reference: src/test/java/g3101_3200/s3158_find_the_xor_of_numbers_which_appear_twice/SolutionTest.java

use leetcode_in_rust::s3158::find_the_xor_of_numbers_which_appear_twice::Solution;
#[test]
fn test_duplicate_numbers_xor() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]), 1);
}
#[test]
fn test_duplicate_numbers_xor2() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 3]), 0);
}
#[test]
fn test_duplicate_numbers_xor3() {
    assert_eq!(Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]), 3);
}
