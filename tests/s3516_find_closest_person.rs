// Tests for Problem 3516: Find Closest Person
// Java reference: src/test/java/g3501_3600/s3516_find_closest_person/SolutionTest.java

use leetcode_in_rust::s3516::find_closest_person::Solution;

#[test]
fn test_find_closest() {
    assert_eq!(Solution::find_closest(2, 7, 4), 1);
}

#[test]
fn test_find_closest2() {
    assert_eq!(Solution::find_closest(2, 5, 6), 2);
}

#[test]
fn test_find_closest3() {
    assert_eq!(Solution::find_closest(1, 5, 3), 0);
}
