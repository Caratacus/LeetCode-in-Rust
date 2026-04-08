// Tests for Problem 3132: Find the Integer Added to Array II
// Java reference: src/test/java/g3101_3200/s3132_find_the_integer_added_to_array_ii/SolutionTest.java

use leetcode_in_rust::s3132::find_the_integer_added_to_array_ii::Solution;
#[test]
fn test_minimum_added_integer() {
    assert_eq!(Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]), -2);
}

#[test]
fn test_minimum_added_integer2() {
    assert_eq!(Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7]), 2);
}

#[test]
fn test_minimum_added_integer3() {
    assert_eq!(Solution::minimum_added_integer(vec![10, 20, 30, 40], vec![35, 45]), 5);
}
#[test]
fn test_minimum_added_integer4() {
    assert_eq!(Solution::minimum_added_integer(vec![2, 5, 7, 8], vec![9, 10]), 2);
}
#[test]
fn test_minimum_added_integer5() {
    assert_eq!(Solution::minimum_added_integer(vec![2, 2, 2, 4, 4], vec![6, 6, 8]), 4);
}
#[test]
fn test_minimum_added_integer6() {
    assert_eq!(Solution::minimum_added_integer(vec![5, 5, 5, 5], vec![5, 5]), 0);
}
#[test]
fn test_minimum_added_integer7() {
    assert_eq!(
        Solution::minimum_added_integer(vec![1_000_000, 2_000_000, 3_000_000, 4_000_000], vec![3_000_002, 4_000_002]),
        2
    );
}
