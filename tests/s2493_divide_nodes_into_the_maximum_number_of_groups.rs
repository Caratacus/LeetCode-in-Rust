// Tests for Problem 2493: Divide Nodes Into the Maximum Number of Groups
// Java reference: src/test/java/g2401_2500/s2493_divide_nodes_into_the_maximum_number_of_groups/SolutionTest.java

use leetcode_in_rust::s2493::divide_nodes_into_the_maximum_number_of_groups::Solution;

#[test]
fn test_magnificent_sets() {
    assert_eq!(
        Solution::magnificent_sets(6, vec![vec![1, 2], vec![1, 4], vec![1, 5], vec![2, 6], vec![2, 3], vec![4, 6]]),
        4
    );
}

#[test]
fn test_magnificent_sets2() {
    assert_eq!(
        Solution::magnificent_sets(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]),
        -1
    );
}
