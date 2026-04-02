// Tests for Problem 0790: Domino and Tromino Tiling
// Java reference: src/test/java/g0701_0800/s0790_domino_and_tromino_tiling/SolutionTest.java

use leetcode_in_rust::s0790::domino_and_tromino_tiling::Solution;

#[test]
fn test_num_tilings() {
    assert_eq!(Solution::num_tilings(3), 5);
}

#[test]
fn test_num_tilings2() {
    assert_eq!(Solution::num_tilings(1), 1);
}

#[test]
fn test_num_tilings3() {
    assert_eq!(Solution::num_tilings(2), 2);
}

#[test]
fn test_num_tilings4() {
    assert_eq!(Solution::num_tilings(4), 11);
}

#[test]
fn test_num_tilings5() {
    assert_eq!(Solution::num_tilings(5), 24);
}

#[test]
fn test_num_tilings6() {
    assert_eq!(Solution::num_tilings(6), 53);
}
