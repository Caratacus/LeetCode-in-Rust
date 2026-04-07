// Tests for Problem 2101: Detonate the Maximum Bombs
// Java reference: src/test/java/g2101_2200/s2101_detonate_the_maximum_bombs/SolutionTest.java

use leetcode_in_rust::s2101::detonate_the_maximum_bombs::Solution;

#[test]
fn test_maximum_detonation() {
    assert_eq!(
        Solution::maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]),
        2
    );
}

#[test]
fn test_maximum_detonation2() {
    assert_eq!(
        Solution::maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]]),
        1
    );
}

#[test]
fn test_maximum_detonation3() {
    assert_eq!(
        Solution::maximum_detonation(vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 4, 2],
            vec![4, 5, 3],
            vec![5, 6, 4]
        ]),
        5
    );
}
