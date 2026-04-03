// Tests for Problem 1622: Fancy Sequence
// Java reference: src/test/java/g1601_1700/s1622_fancy_sequence/FancyTest.java

use leetcode_in_rust::s1622::fancy_sequence::Fancy;

#[test]
fn test_fancy_test() {
    let mut fancy = Fancy::new();
    fancy.append(2);
    fancy.add_all(3);
    fancy.append(7);
    fancy.mult_all(2);
    assert_eq!(fancy.get_index(0), 10);
    fancy.add_all(3);
    fancy.append(10);
    fancy.mult_all(2);
    assert_eq!(fancy.get_index(0), 26);
    assert_eq!(fancy.get_index(1), 34);
    assert_eq!(fancy.get_index(2), 20);
}
