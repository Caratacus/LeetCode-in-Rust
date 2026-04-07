// Tests for Problem 2741: Special Permutations
// Java reference: src/test/java/g2701_2800/s2741_special_permutations/SolutionTest.java

use leetcode_in_rust::s2741::special_permutations::Solution;

#[test]
fn test_special_perm() {
    assert_eq!(Solution::special_perm(vec![2, 3, 6]), 2);
}

#[test]
fn test_special_perm2() {
    assert_eq!(Solution::special_perm(vec![1, 4, 3]), 2);
}
