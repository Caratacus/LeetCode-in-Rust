// Tests for Problem 3245: Alternating Groups III
// Java reference: src/test/java/g3201_3300/s3245_alternating_groups_iii/SolutionTest.java

use leetcode_in_rust::s3245::alternating_groups_iii::Solution;

#[test]
fn test_number_of_alternating_groups() {
    assert_eq!(
        Solution::number_of_alternating_groups(
            vec![0, 1, 1, 0, 1],
            vec![vec![2, 1, 0], vec![1, 4]]
        ),
        vec![2]
    );
}

#[test]
fn test_number_of_alternating_groups2() {
    assert_eq!(
        Solution::number_of_alternating_groups(
            vec![0, 0, 1, 0, 1, 1],
            vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]]
        ),
        vec![6, 4]
    );
}

#[test]
fn test_number_of_alternating_groups3() {
    assert_eq!(
        Solution::number_of_alternating_groups(
            vec![0, 0, 0, 1],
            vec![vec![2, 1, 1], vec![1, 3], vec![2, 1, 1], vec![2, 0, 1]]
        ),
        vec![2, 2, 2]
    );
}
