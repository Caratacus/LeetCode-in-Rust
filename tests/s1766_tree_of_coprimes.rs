// Tests for Problem 1766: Tree of Coprimes
// Java reference: src/test/java/g1701_1800/s1766_tree_of_coprimes/SolutionTest.java

use leetcode_in_rust::s1766::tree_of_coprimes::Solution;

#[test]
fn test_get_coprimes() {
    assert_eq!(
        Solution::get_coprimes(vec![2, 3, 3, 2], vec![vec![0, 1], vec![1, 2], vec![1, 3]]),
        vec![-1, 0, 0, 1]
    );
}

#[test]
fn test_get_coprimes2() {
    assert_eq!(
        Solution::get_coprimes(
            vec![5, 6, 10, 2, 3, 6, 15],
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![1, 4], vec![2, 5], vec![2, 6]]
        ),
        vec![-1, 0, -1, 0, 0, 0, -1]
    );
}
