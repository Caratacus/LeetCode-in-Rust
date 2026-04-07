// Tests for Problem 2592: Maximize Greatness of an Array
// Java reference: src/test/java/g2501_2600/s2592_maximize_greatness_of_an_array/SolutionTest.java

use leetcode_in_rust::s2592::maximize_greatness_of_an_array::Solution;

#[test]
fn test_maximize_greatness() {
    assert_eq!(Solution::maximize_greatness(vec![1, 3, 5, 2, 1, 3, 1]), 4);
}

#[test]
fn test_maximize_greatness2() {
    assert_eq!(Solution::maximize_greatness(vec![1, 2, 3, 4]), 3);
}
