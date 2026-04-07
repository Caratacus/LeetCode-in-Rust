// Tests for Problem 2245: Maximum Trailing Zeros in a Cornered Path
// Java reference: src/test/java/g2201_2300/s2245_maximum_trailing_zeros_in_a_cornered_path/SolutionTest.java

use leetcode_in_rust::s2245::maximum_trailing_zeros_in_a_cornered_path::Solution;

#[test]
fn test_max_trailing_zeros() {
    assert_eq!(
        Solution::max_trailing_zeros(vec![
            vec![23, 17, 15, 3, 20],
            vec![8, 1, 20, 27, 11],
            vec![9, 4, 6, 2, 21],
            vec![40, 9, 1, 10, 6],
            vec![22, 7, 4, 5, 3]
        ]),
        3
    );
}

#[test]
fn test_max_trailing_zeros2() {
    assert_eq!(
        Solution::max_trailing_zeros(vec![vec![4, 3, 2], vec![7, 6, 1], vec![8, 8, 8]]),
        0
    );
}
