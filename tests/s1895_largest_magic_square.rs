// Tests for Problem 1895: Largest Magic Square
// Java reference: src/test/java/g1801_1900/s1895_largest_magic_square/SolutionTest.java

use leetcode_in_rust::s1895::largest_magic_square::Solution;

#[test]
fn test_largest_magic_square() {
    assert_eq!(
        Solution::largest_magic_square(vec![
            vec![7, 1, 4, 5, 6],
            vec![2, 5, 1, 6, 4],
            vec![1, 9, 2, 4, 5],
            vec![3, 6, 2, 5, 1],
            vec![2, 7, 1, 4, 3]
        ]),
        3
    );
}

#[test]
fn test_largest_magic_square2() {
    assert_eq!(
        Solution::largest_magic_square(vec![
            vec![5, 1, 3, 1],
            vec![9, 3, 3, 1],
            vec![1, 3, 3, 8]
        ]),
        2
    );
}
