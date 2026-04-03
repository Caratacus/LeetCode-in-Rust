// Tests for Problem 1792: Maximum Average Pass Ratio
// Java reference: src/test/java/g1701_1800/s1792_maximum_average_pass_ratio/SolutionTest.java

use leetcode_in_rust::s1792::maximum_average_pass_ratio::Solution;

#[test]
fn test_max_average_ratio() {
    let result = Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2);
    assert!((result - 0.7833333333333333).abs() < 1e-9);
}

#[test]
fn test_max_average_ratio2() {
    let result = Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4);
    assert!((result - 0.5348484848484849).abs() < 1e-9);
}
