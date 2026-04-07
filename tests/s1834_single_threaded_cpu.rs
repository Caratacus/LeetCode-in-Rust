// Tests for Problem 1834: Single-Threaded CPU
// Java reference: src/test/java/g1801_1900/s1834_single_threaded_cpu/SolutionTest.java

use leetcode_in_rust::s1834::single_threaded_cpu::Solution;

#[test]
fn test_get_order() {
    assert_eq!(
        Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
        vec![0, 2, 3, 1]
    );
}

#[test]
fn test_get_order2() {
    assert_eq!(
        Solution::get_order(vec![
            vec![7, 10],
            vec![7, 12],
            vec![7, 5],
            vec![7, 4],
            vec![7, 2]
        ]),
        vec![4, 3, 2, 0, 1]
    );
}

#[test]
fn test_get_order3() {
    assert_eq!(
        Solution::get_order(vec![vec![1, 2], vec![2, 4], vec![3, 2], vec![4, 1]]),
        vec![0, 2, 3, 1]
    );
}
