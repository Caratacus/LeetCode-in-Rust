// Tests for Problem 3040: Maximum Number of Operations With the Same Score II
// Java reference: src/test/java/g3001_3100/s3040_maximum_number_of_operations_with_the_same_score_ii/SolutionTest.java

use leetcode_in_rust::s3040::maximum_number_of_operations_with_the_same_score_ii::Solution;

#[test]
fn test_max_operations() {
    assert_eq!(Solution::max_operations(vec![3, 2, 1, 2, 3, 4]), 3);
}

#[test]
fn test_max_operations2() {
    assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 2);
}

#[test]
fn test_max_operations3() {
    assert_eq!(Solution::max_operations(vec![1, 2]), 1);
}

#[test]
fn test_max_operations4() {
    assert_eq!(Solution::max_operations(vec![1, 1, 1]), 1);
}

#[test]
fn test_max_operations5() {
    assert_eq!(Solution::max_operations(vec![2, 2, 2, 2, 2, 2]), 3);
}

#[test]
fn test_max_operations6() {
    assert_eq!(Solution::max_operations(vec![1, 2, 3, 4, 5, 6]), 3);
}

#[test]
fn test_max_operations7() {
    assert_eq!(Solution::max_operations(vec![6, 5, 4, 3, 2, 1]), 3);
}

#[test]
fn test_max_operations8() {
    assert_eq!(Solution::max_operations(vec![1, 3, 2, 4, 1, 3]), 2);
}

#[test]
fn test_max_operations9() {
    assert_eq!(Solution::max_operations(vec![1, 2, 4, 5]), 2);
}

#[test]
fn test_max_operations10() {
    assert_eq!(Solution::max_operations(vec![5, 5]), 1);
}
