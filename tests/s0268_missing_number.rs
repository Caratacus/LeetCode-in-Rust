// Tests for Problem 0268: Missing Number
// Java reference: src/test/java/g0201_0300/s0268_missing_number/SolutionTest.java

use leetcode_in_rust::s0268::missing_number::Solution;

#[test]
fn test_missing_number() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
}

#[test]
fn test_missing_number2() {
    assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
}

#[test]
fn test_missing_number3() {
    assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
}

#[test]
fn test_missing_number4() {
    assert_eq!(Solution::missing_number(vec![0]), 1);
}
