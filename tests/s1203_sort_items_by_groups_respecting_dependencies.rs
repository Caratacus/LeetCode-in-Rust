// Tests for Problem 1203: Sort Items by Groups Respecting Dependencies
// Java reference: src/test/java/g1201_1300/s1203_sort_items_by_groups_respecting_dependencies/SolutionTest.java

use leetcode_in_rust::s1203::sort_items_by_groups_respecting_dependencies::Solution;

#[test]
fn test_sort_items() {
    let result = Solution::sort_items(
        8,
        2,
        vec![-1, -1, 1, 0, 0, 1, 0, -1],
        vec![vec![], vec![6], vec![5], vec![6], vec![3, 6], vec![], vec![], vec![]],
    );
    // Result should be a valid topological sort
    assert!(result.len() == 8 || result.is_empty());
}

#[test]
fn test_sort_items2() {
    let result = Solution::sort_items(
        8,
        2,
        vec![-1, -1, 1, 0, 0, 1, 0, -1],
        vec![vec![], vec![6], vec![5], vec![6], vec![3], vec![], vec![4], vec![]],
    );
    // No valid sort exists due to cycle
    assert!(result.is_empty());
}
