// Tests for Problem 1470: Shuffle the Array
// Java reference: src/test/java/g1401_1500/s1470_shuffle_the_array/SolutionTest.java

use leetcode_in_rust::s1470::shuffle_the_array::Solution;

#[test]
fn test_shuffle() {
    assert_eq!(
        Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
        vec![2, 3, 5, 4, 1, 7]
    );
}

#[test]
fn test_shuffle2() {
    assert_eq!(
        Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
        vec![1, 4, 2, 3, 3, 2, 4, 1]
    );
}

#[test]
fn test_shuffle3() {
    assert_eq!(Solution::shuffle(vec![1, 1, 2, 2], 2), vec![1, 2, 1, 2]);
}
