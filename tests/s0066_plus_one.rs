// Tests for Problem 0066: Plus One
// Java reference: src/test/java/g0001_0100/s0066_plus_one/SolutionTest.java

use leetcode_in_rust::s0066::plus_one::Solution;

#[test]
fn test_plus_one() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
}

#[test]
fn test_plus_one2() {
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
}

#[test]
fn test_plus_one3() {
    assert_eq!(Solution::plus_one(vec![0]), vec![1]);
}

#[test]
fn test_plus_one4() {
    assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
}
