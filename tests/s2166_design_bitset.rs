// Tests for Problem 2166: Design Bitset
// Java reference: src/test/java/g2101_2200/s2166_design_bitset/BitsetTest.java

use leetcode_in_rust::s2166::design_bitset::Bitset;

#[test]
fn test_bitset() {
    // bitset = "00000".
    let mut bs = Bitset::new(5);
    // the value at idx = 3 is updated to 1, so bitset = "00010".
    bs.fix(3);
    // the value at idx = 1 is updated to 1, so bitset = "01010".
    bs.fix(1);
    // the value of each bit is flipped, so bitset = "10101".
    bs.flip();
    // return False, as not all values of the bitset are 1.
    assert!(!bs.all());
    // the value at idx = 0 is updated to 0, so bitset = "00101".
    bs.unfix(0);
    // the value of each bit is flipped, so bitset = "11010".
    bs.flip();
    // return True, as there is at least 1 index with value 1.
    assert!(bs.one());
    // the value at idx = 0 is updated to 0, so bitset = "01010".
    bs.unfix(0);
    // return 2, as there are 2 bits with value 1.
    assert_eq!(bs.count(), 2);
    // return "01010", which is the composition of bitset.
    assert_eq!(bs.to_string(), "01010");
}
