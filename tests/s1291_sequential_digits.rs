// Tests for Problem 1291: Sequential Digits
// Java reference: src/test/java/g1201_1300/s1291_sequential_digits/SolutionTest.java

use leetcode_in_rust::s1291::sequential_digits::Solution;

#[test]
fn test_sequential_digits() {
    assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
}

#[test]
fn test_sequential_digits2() {
    assert_eq!(
        Solution::sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}
