// Tests for Problem 3619: Count Islands with Total Value Divisible by K
// Java reference: src/test/java/g3601_3700/s3619_count_islands_with_total_value_divisible_by_k/SolutionTest.java
use leetcode_in_rust::s3619::count_islands_with_total_value_divisible_by_k::Solution;
#[test]
fn test_count_islands() {
    assert_eq!(
        Solution::count_islands(
            vec![vec![0, 2, 1, 0, 0], vec![0, 5, 0, 0, 5], vec![0, 0, 1, 0, 0], vec![0, 1, 4, 7, 0], vec![0, 2, 0, 0, 8]],
            5
        ),
        2
    );
}
#[test]
fn test_count_islands2() {
    assert_eq!(Solution::count_islands(vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]], 3), 6);
}
