// Tests for Problem 3131: Find the Integer Added to Array I
// Java reference: src/test/java/g3101_3200/s3131_find_the_integer_added_to_array_i/SolutionTest.java

use leetcode_in_rust::s3131::find_the_integer_added_to_array_i::Solution;

#[test]
fn test_added_integer() {
    assert_eq!(Solution::added_integer(vec![2, 6, 4], vec![9, 7, 5]), 3);
}

#[test]
fn test_added_integer2() {
    assert_eq!(Solution::added_integer(vec![10], vec![5]), -5);
}
#[test]
fn test_added_integer3() {
    assert_eq!(Solution::added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), 0);
}
