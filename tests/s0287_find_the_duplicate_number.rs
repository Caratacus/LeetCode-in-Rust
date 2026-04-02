// Tests for Problem 0287: Find the Duplicate Number
// Java reference: src/test/java/g0201_0300/s0287_find_the_duplicate_number/SolutionTest.java

use leetcode_in_rust::s0287::find_the_duplicate_number::Solution;

#[test]
fn test_find_duplicate() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
}

#[test]
fn test_find_duplicate2() {
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
}

#[test]
fn test_find_duplicate3() {
    assert_eq!(Solution::find_duplicate(vec![1, 1]), 1);
}

#[test]
fn test_find_duplicate4() {
    assert_eq!(Solution::find_duplicate(vec![1, 1, 2]), 1);
}
