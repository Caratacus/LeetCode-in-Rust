// Tests for Problem 3270: Find the Key of the Numbers
// Java reference: src/test/java/g3201_3300/s3270_find_the_key_of_the_numbers/SolutionTest.java

use leetcode_in_rust::s3270::find_the_key_of_the_numbers::Solution;

#[test]
fn test_generate_key() {
    assert_eq!(Solution::generate_key(1, 10, 1000), 0);
}

#[test]
fn test_generate_key2() {
    assert_eq!(Solution::generate_key(987, 879, 798), 777);
}

#[test]
fn test_generate_key3() {
    assert_eq!(Solution::generate_key(1, 2, 3), 1);
}
