// Tests for Problem 1286: Iterator for Combination
// Java reference: src/test/java/g1201_1300/s1286_iterator_for_combination/SolutionTest.java

use leetcode_in_rust::s1286::iterator_for_combination::CombinationIterator;

#[test]
fn test_combination_iterator() {
    let mut itr = CombinationIterator::new("abc".to_string(), 2);
    assert_eq!(itr.next(), "ab");
    assert_eq!(itr.has_next(), true);
    assert_eq!(itr.next(), "ac");
    assert_eq!(itr.has_next(), true);
    assert_eq!(itr.next(), "bc");
    assert_eq!(itr.has_next(), false);
}
