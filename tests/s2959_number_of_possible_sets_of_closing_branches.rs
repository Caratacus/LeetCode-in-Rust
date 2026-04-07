// Tests for Problem 2959: Number of Possible Sets of Closing Branches
// Java reference: src/test/java/g2901_3000/s2959_number_of_possible_sets_of_closing_branches/SolutionTest.java

use leetcode_in_rust::s2959::number_of_possible_sets_of_closing_branches::Solution;

#[test]
fn test_number_of_sets() {
    assert_eq!(
        Solution::number_of_sets(3, 5, vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]]),
        5
    );
}

#[test]
fn test_number_of_sets2() {
    assert_eq!(
        Solution::number_of_sets(3, 5, vec![vec![0, 1, 20], vec![0, 1, 10], vec![1, 2, 2], vec![0, 2, 2]]),
        7
    );
}

#[test]
fn test_number_of_sets3() {
    assert_eq!(Solution::number_of_sets(1, 10, vec![] as Vec<Vec<i32>>), 2);
}
