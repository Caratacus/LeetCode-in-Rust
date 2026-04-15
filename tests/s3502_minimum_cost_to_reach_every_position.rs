// Tests for Problem 3502: Minimum Cost to Reach Every Position
// Java reference: src/test/java/g3501_3600/s3502_minimum_cost_to_reach_every_position/SolutionTest.java

use leetcode_in_rust::s3502::minimum_cost_to_reach_every_position::Solution;

#[test]
fn test_min_costs() {
    assert_eq!(Solution::min_costs(vec![5, 3, 4, 1, 3, 2]), vec![5, 3, 3, 1, 1, 1]);
}

#[test]
fn test_min_costs2() {
    assert_eq!(Solution::min_costs(vec![1, 2, 4, 6, 7]), vec![1, 1, 1, 1, 1]);
}
