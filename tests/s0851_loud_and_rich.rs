// Tests for Problem 0851: Loud and Rich
// Java reference: src/test/java/g0801_0900/s0851_loud_and_rich/SolutionTest.java

use leetcode_in_rust::s0851::loud_and_rich::Solution;

#[test]
fn test_loud_and_rich() {
    assert_eq!(
        Solution::loud_and_rich(
            vec![vec![1, 0], vec![2, 1], vec![3, 1], vec![3, 7], vec![4, 3], vec![5, 3], vec![6, 3]],
            vec![3, 2, 5, 4, 6, 1, 7, 0]
        ),
        vec![5, 5, 2, 5, 4, 5, 6, 7]
    );
}

#[test]
fn test_loud_and_rich2() {
    assert_eq!(
        Solution::loud_and_rich(vec![], vec![0]),
        vec![0]
    );
}
