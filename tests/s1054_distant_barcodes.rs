// Tests for Problem 1054: Distant Barcodes
// Java reference: src/test/java/g1001_1100/s1054_distant_barcodes/SolutionTest.java

use leetcode_in_rust::s1054::distant_barcodes::Solution;

#[test]
fn test_rearrange_barcodes() {
    assert_eq!(
        Solution::rearrange_barcodes(vec![1, 1, 1, 2, 2, 2]),
        vec![1, 2, 1, 2, 1, 2]
    );
}

#[test]
fn test_rearrange_barcodes2() {
    assert_eq!(
        Solution::rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3]),
        vec![1, 2, 1, 2, 1, 3, 1, 3]
    );
}
