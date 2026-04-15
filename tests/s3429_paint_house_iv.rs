// Tests for Problem 3429: Paint House IV
// Java reference: src/test/java/g3401_3500/s3429_paint_house_iv/SolutionTest.java

use leetcode_in_rust::s3429::paint_house_iv::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(
        Solution::min_cost(4, vec![vec![3, 5, 7], vec![6, 2, 9], vec![4, 8, 1], vec![7, 3, 5]]),
        9i64
    );
}

#[test]
fn test_min_cost2() {
    assert_eq!(
        Solution::min_cost(
            6,
            vec![vec![2, 4, 6], vec![5, 3, 8], vec![7, 1, 9], vec![4, 6, 2], vec![3, 5, 7], vec![8, 2, 4]]
        ),
        18i64
    );
}
