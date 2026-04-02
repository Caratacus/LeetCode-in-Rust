// Tests for Problem 0957: Prison Cells After N Days
// Java reference: src/test/java/g0901_1000/s0957_prison_cells_after_n_days/SolutionTest.java

use leetcode_in_rust::s0957::prison_cells_after_n_days::Solution;

#[test]
fn test_prison_after_n_days() {
    assert_eq!(
        Solution::prison_after_n_days(vec![0, 1, 0, 1, 1, 0, 0, 1], 7),
        vec![0, 0, 1, 1, 0, 0, 0, 0]
    );
}

#[test]
fn test_prison_after_n_days2() {
    assert_eq!(
        Solution::prison_after_n_days(vec![1, 0, 0, 1, 0, 0, 1, 0], 1000000000),
        vec![0, 0, 1, 1, 1, 1, 1, 0]
    );
}
