// Tests for Problem 3151: Special Array I
// Java reference: src/test/java/g3101_3200/s3151_special_array_i/SolutionTest.java

use leetcode_in_rust::s3151::special_array_i::Solution;
#[test]
fn test_is_array_special() {
    assert_eq!(Solution::is_array_special(vec![1]), true);
}
#[test]
fn test_is_array_special2() {
    assert_eq!(Solution::is_array_special(vec![2, 1, 4]), true);
}
#[test]
fn test_is_array_special3() {
    assert_eq!(Solution::is_array_special(vec![4, 3, 1, 6]), false);
}
#[test]
fn test_is_array_special4() {
    assert_eq!(Solution::is_array_special(vec![2, 10]), false);
}
