// Tests for Problem 2336: Smallest Number in Infinite Set
// Java reference: src/test/java/g2301_2400/s2336_smallest_number_in_infinite_set/SmallestInfiniteSetTest.java

use leetcode_in_rust::s2336::smallest_number_in_infinite_set::SmallestInfiniteSet;

#[test]
fn test_smallest_infinite_set() {
    let mut smallest_infinite_set = SmallestInfiniteSet::new();
    // 2 is already in the set, so no change is made.
    smallest_infinite_set.add_back(2);
    // return 1, since 1 is the smallest number, and remove it from the set.
    assert_eq!(smallest_infinite_set.pop_smallest(), 1);
    // return 2, and remove it from the set.
    assert_eq!(smallest_infinite_set.pop_smallest(), 2);
    // return 3, and remove it from the set.
    assert_eq!(smallest_infinite_set.pop_smallest(), 3);
    // 1 is added back to the set.
    smallest_infinite_set.add_back(1);
    // return 1, since 1 was added back to the set and is the smallest number, and remove it from the set.
    assert_eq!(smallest_infinite_set.pop_smallest(), 1);
    // return 4, and remove it from the set.
    assert_eq!(smallest_infinite_set.pop_smallest(), 4);
    // return 5, and remove it from the set.
    assert_eq!(smallest_infinite_set.pop_smallest(), 5);
}
