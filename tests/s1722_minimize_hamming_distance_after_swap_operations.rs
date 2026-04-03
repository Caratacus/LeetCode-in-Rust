// Tests for Problem 1722: Minimize Hamming Distance After Swap Operations
// Java reference: src/test/java/g1701_1800/s1722_minimize_hamming_distance_after_swap_operations/SolutionTest.java

use leetcode_in_rust::s1722::minimize_hamming_distance_after_swap_operations::Solution;

#[test]
fn test_minimum_hamming_distance() {
    assert_eq!(
        Solution::minimum_hamming_distance(
            vec![1, 2, 3, 4],
            vec![2, 1, 4, 5],
            vec![vec![0, 1], vec![2, 3]]
        ),
        1
    );
}

#[test]
fn test_minimum_hamming_distance2() {
    assert_eq!(
        Solution::minimum_hamming_distance(
            vec![5, 1, 2, 4, 3],
            vec![1, 5, 4, 2, 3],
            vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]]
        ),
        0
    );
}

#[test]
fn test_minimum_hamming_distance3() {
    assert_eq!(
        Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![]),
        2
    );
}
