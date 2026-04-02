// Tests for Problem 1094: Car Pooling
// Java reference: src/test/java/g1001_1100/s1094_car_pooling/SolutionTest.java

use leetcode_in_rust::s1094::car_pooling::Solution;

#[test]
fn test_car_pooling() {
    assert_eq!(
        Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4),
        false
    );
}

#[test]
fn test_car_pooling2() {
    assert_eq!(
        Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5),
        true
    );
}
