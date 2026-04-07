// Tests for Problem 2242: Maximum Score of a Node Sequence
// Java reference: src/test/java/g2201_2300/s2242_maximum_score_of_a_node_sequence/SolutionTest.java

use leetcode_in_rust::s2242::maximum_score_of_a_node_sequence::Solution;

#[test]
fn test_maximum_score() {
    assert_eq!(
        Solution::maximum_score(
            vec![5, 2, 9, 8, 4],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]]
        ),
        24
    );
}

#[test]
fn test_maximum_score2() {
    assert_eq!(
        Solution::maximum_score(
            vec![9, 20, 6, 4, 11, 12],
            vec![vec![0, 3], vec![5, 3], vec![2, 4], vec![1, 3]]
        ),
        -1
    );
}

#[test]
fn test_maximum_score3() {
    assert_eq!(
        Solution::maximum_score(
            vec![14, 12, 10, 8, 1, 2, 3, 1],
            vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![4, 0], vec![5, 1], vec![6, 1], vec![7, 1], vec![2, 1]]
        ),
        44
    );
}

#[test]
fn test_maximum_score4() {
    assert_eq!(
        Solution::maximum_score(
            vec![29, 28, 16, 28, 15, 27, 17, 10, 17],
            vec![vec![6, 2], vec![2, 4], vec![4, 7], vec![3, 2], vec![6, 4], vec![7, 3], vec![3, 6], vec![6, 1], vec![2, 5], vec![5, 7], vec![7, 6], vec![1, 4], vec![3, 0], vec![0, 4], vec![5, 4], vec![4, 8], vec![8, 0], vec![0, 5], vec![1, 5]]
        ),
        112
    );
}
