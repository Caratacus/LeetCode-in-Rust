// Tests for Problem 2717: Semi Ordered Permutation
// Java reference: src/test/java/g2701_2800/s2717_semi_ordered_permutation/SolutionTest.java

use leetcode_in_rust::s2717::semi_ordered_permutation::Solution;

#[test]
fn test_semi_ordered_permutation() {
    assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
}

#[test]
fn test_semi_ordered_permutation2() {
    assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
}

#[test]
fn test_semi_ordered_permutation3() {
    assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
}
