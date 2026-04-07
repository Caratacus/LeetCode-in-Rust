// Tests for Problem 2420: Find All Good Indices
// Java reference: src/test/java/g2401_2500/s2420_find_all_good_indices/SolutionTest.java

use leetcode_in_rust::s2420::find_all_good_indices::Solution;

#[test]
fn test_good_indices() {
    assert_eq!(
        Solution::good_indices(vec![2, 1, 1, 1, 3, 4, 1], 2),
        vec![2, 3]
    );
}

#[test]
fn test_good_indices2() {
    assert_eq!(
        Solution::good_indices(vec![2, 1, 1, 2], 2),
        Vec::<i32>::new()
    );
}

#[test]
fn test_good_indices3() {
    assert_eq!(
        Solution::good_indices(vec![5, 3, 4, 2, 1], 1),
        vec![1, 2, 3]
    );
}

#[test]
fn test_good_indices4() {
    assert_eq!(
        Solution::good_indices(vec![5, 4, 3, 2, 1], 2),
        Vec::<i32>::new()
    );
}

#[test]
fn test_good_indices5() {
    assert_eq!(
        Solution::good_indices(vec![1, 2, 3, 4, 5], 2),
        Vec::<i32>::new()
    );
}

#[test]
fn test_good_indices6() {
    assert_eq!(
        Solution::good_indices(vec![1, 2], 2),
        Vec::<i32>::new()
    );
}

#[test]
fn test_good_indices7() {
    assert_eq!(
        Solution::good_indices(vec![5, 4, 4, 3, 2, 2, 3, 4, 4, 3, 2], 2),
        vec![3, 4, 5, 6]
    );
}

#[test]
fn test_good_indices8() {
    assert_eq!(
        Solution::good_indices(vec![3, 2, 1, 2, 3, 2, 1, 2, 3], 2),
        vec![2, 6]
    );
}

#[test]
fn test_good_indices9() {
    assert_eq!(
        Solution::good_indices(vec![3, 2, 1, 1, 2, 3], 2),
        vec![2, 3]
    );
}

#[test]
fn test_good_indices10() {
    assert_eq!(
        Solution::good_indices(vec![2, 2, 2, 2, 2], 2),
        vec![2]
    );
}
