// Tests for Problem 3447: Assign Elements to Groups with Constraints
// Java reference: src/test/java/g3401_3500/s3447_assign_elements_to_groups_with_constraints/SolutionTest.java

use leetcode_in_rust::s3447::assign_elements_to_groups_with_constraints::Solution;

#[test]
fn test_assign_elements() {
    assert_eq!(
        Solution::assign_elements(vec![8, 4, 3, 2, 4], vec![4, 2]),
        vec![0, 0, -1, 1, 0]
    );
}

#[test]
fn test_assign_elements2() {
    assert_eq!(
        Solution::assign_elements(vec![2, 3, 5, 7], vec![5, 3, 3]),
        vec![-1, 1, 0, -1]
    );
}

#[test]
fn test_assign_elements3() {
    assert_eq!(
        Solution::assign_elements(vec![10, 21, 30, 41], vec![2, 1]),
        vec![0, 1, 0, 1]
    );
}
