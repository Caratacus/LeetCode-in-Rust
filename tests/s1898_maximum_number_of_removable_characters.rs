// Tests for Problem 1898: Maximum Number of Removable Characters
// Java reference: src/test/java/g1801_1900/s1898_maximum_number_of_removable_characters/SolutionTest.java

use leetcode_in_rust::s1898::maximum_number_of_removable_characters::Solution;

#[test]
fn test_maximum_removals() {
    assert_eq!(
        Solution::maximum_removals("abcacb".to_string(), "ab".to_string(), vec![3, 1, 0]),
        2
    );
}

#[test]
fn test_maximum_removals2() {
    assert_eq!(
        Solution::maximum_removals("abcbddddd".to_string(), "abcd".to_string(), vec![3, 2, 1, 4, 5, 6]),
        1
    );
}
