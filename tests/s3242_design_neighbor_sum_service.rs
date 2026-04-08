// Tests for Problem 3242: Design Neighbor Sum Service
// Java reference: src/test/java/g3201_3300/s3242_design_neighbor_sum_service/SolutionTest.java

use leetcode_in_rust::s3242::design_neighbor_sum_service::NeighborSum;

#[test]
fn test_neighbor_sum() {
    let mut obj = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
    assert_eq!(obj.adjacent_sum(4), 13);
    assert_eq!(obj.diagonal_sum(4), 20);
    assert_eq!(obj.adjacent_sum(8), 15);
    assert_eq!(obj.diagonal_sum(8), 4);
}

#[test]
fn test_neighbor_sum2() {
    let mut obj = NeighborSum::new(vec![vec![1, 2], vec![3, 4]]);
    assert_eq!(obj.adjacent_sum(1), 5);
    assert_eq!(obj.diagonal_sum(1), 4);
    assert_eq!(obj.adjacent_sum(4), 5);
    assert_eq!(obj.diagonal_sum(4), 2);
}
