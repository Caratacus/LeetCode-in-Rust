// Tests for Problem 3531: Count Covered Buildings
// Java reference: src/test/java/g3501_3600/s3531_count_covered_buildings/SolutionTest.java

use leetcode_in_rust::s3531::count_covered_buildings::Solution;

#[test]
fn test_count_covered_buildings() {
    assert_eq!(Solution::count_covered_buildings(3, vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]]), 1);
}

#[test]
fn test_count_covered_buildings2() {
    assert_eq!(Solution::count_covered_buildings(3, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]), 0);
}

#[test]
fn test_count_covered_buildings3() {
    assert_eq!(Solution::count_covered_buildings(5, vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]]), 1);
}
