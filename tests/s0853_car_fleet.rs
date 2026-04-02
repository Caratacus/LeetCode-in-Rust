// Tests for Problem 0853: Car Fleet
// Java reference: src/test/java/g0801_0900/s0853_car_fleet/SolutionTest.java

use leetcode_in_rust::s0853::car_fleet::Solution;

#[test]
fn test_car_fleet() {
    assert_eq!(
        Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
        3
    );
}

#[test]
fn test_car_fleet2() {
    assert_eq!(
        Solution::car_fleet(10, vec![3], vec![3]),
        1
    );
}

#[test]
fn test_car_fleet3() {
    assert_eq!(
        Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]),
        1
    );
}
