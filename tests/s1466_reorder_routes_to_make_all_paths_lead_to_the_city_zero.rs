// Tests for Problem 1466: Reorder Routes to Make All Paths Lead to the City Zero
// Java reference: src/test/java/g1401_1500/s1466_reorder_routes_to_make_all_paths_lead_to_the_city_zero/SolutionTest.java

use leetcode_in_rust::s1466::reorder_routes_to_make_all_paths_lead_to_the_city_zero::Solution;

#[test]
fn test_min_reorder() {
    let connections = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    assert_eq!(Solution::min_reorder(6, connections), 3);
}

#[test]
fn test_min_reorder2() {
    let connections = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
    assert_eq!(Solution::min_reorder(5, connections), 2);
}

#[test]
fn test_min_reorder3() {
    let connections = vec![vec![1, 0], vec![2, 0]];
    assert_eq!(Solution::min_reorder(3, connections), 0);
}
