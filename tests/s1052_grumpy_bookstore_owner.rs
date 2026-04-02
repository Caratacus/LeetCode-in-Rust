// Tests for Problem 1052: Grumpy Bookstore Owner
// Java reference: src/test/java/g1001_1100/s1052_grumpy_bookstore_owner/SolutionTest.java

use leetcode_in_rust::s1052::grumpy_bookstore_owner::Solution;

#[test]
fn test_max_satisfied() {
    assert_eq!(
        Solution::max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        ),
        16
    );
}

#[test]
fn test_max_satisfied2() {
    assert_eq!(Solution::max_satisfied(vec![1], vec![0], 3), 1);
}
