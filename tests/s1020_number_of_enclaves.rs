// Tests for Problem 1020: Number of Enclaves
// Java reference: src/test/java/g1001_1100/s1020_number_of_enclaves/SolutionTest.java

use leetcode_in_rust::s1020::number_of_enclaves::Solution;

#[test]
fn test_num_enclaves() {
    assert_eq!(
        Solution::num_enclaves(vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        3
    );
}

#[test]
fn test_num_enclaves2() {
    assert_eq!(
        Solution::num_enclaves(vec![
            vec![0, 1, 1, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0]
        ]),
        0
    );
}
