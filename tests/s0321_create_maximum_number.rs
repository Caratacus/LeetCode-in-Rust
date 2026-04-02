// Tests for Problem 0321: Create Maximum Number
// Java reference: src/test/java/g0301_0400/s0321_create_maximum_number/SolutionTest.java

use leetcode_in_rust::s0321::create_maximum_number::Solution;

#[test]
fn test_max_number() {
    let mut result = Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5);
    result.sort();
    let mut expected = vec![9, 8, 6, 5, 3];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_max_number2() {
    let mut result = Solution::max_number(vec![6, 7], vec![6, 0, 4], 5);
    result.sort();
    let mut expected = vec![6, 7, 6, 0, 4];
    expected.sort();
    assert_eq!(result, expected);
}
