// Tests for Problem 2768: Number of Black Blocks
// Java reference: src/test/java/g2701_2800/s2768_number_of_black_blocks/SolutionTest.java

use leetcode_in_rust::s2768::number_of_black_blocks::Solution;

#[test]
fn test_count_black_blocks() {
    assert_eq!(
        Solution::count_black_blocks(3, 3, vec![vec![0, 0]]),
        vec![3, 1, 0, 0, 0]
    );
}

#[test]
fn test_count_black_blocks2() {
    assert_eq!(
        Solution::count_black_blocks(3, 3, vec![vec![0, 0], vec![1, 1], vec![0, 2]]),
        vec![0, 2, 2, 0, 0]
    );
}
