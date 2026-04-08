// Tests for Problem 3133: Minimum Array End
// Java reference: src/test/java/g3101_3200/s3133_minimum_array_end/SolutionTest.java

use leetcode_in_rust::s3133::minimum_array_end::Solution;
#[test]
fn test_min_end() {
    assert_eq!(Solution::min_end(3, 4), 6);
}
#[test]
fn test_min_end2() {
    assert_eq!(Solution::min_end(2, 7), 15);
}
