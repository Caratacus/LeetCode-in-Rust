// Tests for Problem 1206: Design Skiplist
// Java reference: src/test/java/g1201_1300/s1206_design_skiplist/SolutionTest.java

use leetcode_in_rust::s1206::design_skiplist::Skiplist;

#[test]
fn test_skiplist() {
    let mut skiplist = Skiplist::new();
    skiplist.add(1);
    skiplist.add(2);
    skiplist.add(3);
    assert_eq!(skiplist.search(0), false);
    assert_eq!(skiplist.add(4), ());
    assert_eq!(skiplist.search(1), true);
    assert_eq!(skiplist.erase(0), false);
    assert_eq!(skiplist.erase(1), true);
    assert_eq!(skiplist.search(1), false);
}
