// Tests for Problem 3378: Count Connected Components in LCM Graph
// Java reference: src/test/java/g3301_3400/s3378_count_connected_components_in_lcm_graph/SolutionTest.java

use leetcode_in_rust::s3378::count_connected_components_in_lcm_graph::Solution;

#[test]
fn test_count_components() {
    assert_eq!(Solution::count_components(vec![2, 4, 8, 3, 9], 5), 4);
}

#[test]
fn test_count_components2() {
    assert_eq!(Solution::count_components(vec![2, 4, 8, 3, 9, 12], 10), 2);
}
