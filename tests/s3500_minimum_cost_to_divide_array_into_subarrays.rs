// Tests for Problem 3500: Minimum Cost to Divide Array into Subarrays
// Java reference: src/test/java/g3401_3500/s3500_minimum_cost_to_divide_array_into_subarrays/SolutionTest.java

use leetcode_in_rust::s3500::minimum_cost_to_divide_array_into_subarrays::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(Solution::minimum_cost(vec![3, 1, 4], vec![4, 6, 6], 1), 110i64);
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(
        Solution::minimum_cost(
            vec![4, 8, 5, 1, 14, 2, 2, 12, 1],
            vec![7, 2, 8, 4, 2, 2, 1, 1, 2],
            7
        ),
        985i64
    );
}
