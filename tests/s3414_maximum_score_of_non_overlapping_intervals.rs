// Tests for Problem 3414: Maximum Score of Non Overlapping Intervals
// Java reference: src/test/java/g3401_3500/s3414_maximum_score_of_non_overlapping_intervals/SolutionTest.java

use leetcode_in_rust::s3414::maximum_score_of_non_overlapping_intervals::Solution;

#[test]
fn test_maximum_weight() {
    assert_eq!(
        Solution::maximum_weight(vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
            vec![6, 9, 3],
            vec![6, 7, 1],
            vec![8, 9, 1]
        ]),
        vec![2, 3]
    );
}

#[test]
fn test_maximum_weight2() {
    assert_eq!(
        Solution::maximum_weight(vec![
            vec![5, 8, 1],
            vec![6, 7, 7],
            vec![4, 7, 3],
            vec![9, 10, 6],
            vec![7, 8, 2],
            vec![11, 14, 3],
            vec![3, 5, 5]
        ]),
        vec![1, 3, 5, 6]
    );
}

#[test]
fn test_maximum_weight3() {
    assert_eq!(
        Solution::maximum_weight(vec![vec![4, 4, 1], vec![2, 5, 3], vec![2, 3, 2]]),
        vec![0, 2]
    );
}

#[test]
fn test_maximum_weight4() {
    assert_eq!(
        Solution::maximum_weight(vec![
            vec![19, 23, 23],
            vec![19, 23, 40],
            vec![1, 16, 31],
            vec![16, 18, 31],
            vec![14, 20, 22],
            vec![14, 22, 5],
            vec![23, 24, 23]
        ]),
        vec![1, 2]
    );
}
