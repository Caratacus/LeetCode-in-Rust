// Tests for Problem 1128: Number of Equivalent Domino Pairs
// Java reference: src/test/java/g1101_1200/s1128_number_of_equivalent_domino_pairs/SolutionTest.java

use leetcode_in_rust::s1128::number_of_equivalent_domino_pairs::Solution;

#[test]
fn test_num_equiv_domino_pairs() {
    assert_eq!(
        Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    );
}

#[test]
fn test_num_equiv_domino_pairs2() {
    assert_eq!(
        Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]]),
        3
    );
}
