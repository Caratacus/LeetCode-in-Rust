// Tests for Problem 2545: Sort the Students by Their Kth Score
// Java reference: src/test/java/g2501_2600/s2545_sort_the_students_by_their_kth_score/SolutionTest.java
use leetcode_in_rust::s2545::sort_the_students_by_their_kth_score::Solution;

#[test]
fn test_sort_the_students() {
    assert_eq!(
        Solution::sort_the_students(
            vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
            2
        ),
        vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]]
    );
}
#[test]
fn test_sort_the_students2() {
    assert_eq!(
        Solution::sort_the_students(vec![vec![3, 4], vec![5, 6]], 0),
        vec![vec![5, 6], vec![3, 4]]
    );
}
