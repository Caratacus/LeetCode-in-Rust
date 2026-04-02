// Tests for Problem 0905: Sort Array By Parity
// Java reference: src/test/java/g0901_1000/s0905_sort_array_by_parity/SolutionTest.java

use leetcode_in_rust::s0905::sort_array_by_parity::Solution;

fn compare_array(arr1: &[i32], arr2: &[i32]) -> bool {
    for i in arr1 {
        if !arr2.contains(i) {
            return false;
        }
    }
    for i in arr2 {
        if !arr1.contains(i) {
            return false;
        }
    }
    true
}

#[test]
fn test_sort_array_by_parity() {
    let result = Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
    assert!(compare_array(&result, &[2, 4, 3, 1]));
}

#[test]
fn test_sort_array_by_parity2() {
    assert_eq!(Solution::sort_array_by_parity(vec![0]), vec![0]);
}
