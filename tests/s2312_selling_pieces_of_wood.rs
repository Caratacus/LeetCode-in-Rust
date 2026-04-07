// Tests for Problem 2312: Selling Pieces of Wood
// Java reference: src/test/java/g2301_2400/s2312_selling_pieces_of_wood/SolutionTest.java

use leetcode_in_rust::s2312::selling_pieces_of_wood::Solution;

#[test]
fn test_selling_wood() {
    assert_eq!(
        Solution::selling_wood(3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]]),
        19
    );
}

#[test]
fn test_selling_wood2() {
    assert_eq!(
        Solution::selling_wood(4, 6, vec![vec![3, 2, 10], vec![1, 4, 2], vec![4, 1, 3]]),
        32
    );
}
