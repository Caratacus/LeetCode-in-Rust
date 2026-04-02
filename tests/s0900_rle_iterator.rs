// Tests for Problem 0900: RLE Iterator
// Java reference: src/test/java/g0801_0900/s0900_rle_iterator/SolutionTest.java

use leetcode_in_rust::s0900::rle_iterator::RLEIterator;

#[test]
fn test_rle_iterator() {
    let mut rle_iterator = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    assert_eq!(rle_iterator.next(2), 8);
    assert_eq!(rle_iterator.next(1), 8);
    assert_eq!(rle_iterator.next(1), 5);
    assert_eq!(rle_iterator.next(2), -1);
}
