// Tests for Problem 0380: Insert Delete GetRandom O(1)
// Java reference: src/test/java/g0301_0400/s0380_insert_delete_getrandom_o1/SolutionTest.java

use leetcode_in_rust::s0380::insert_delete_getrandom_o1::RandomizedSet;

#[test]
fn test_randomized_set() {
    let mut set = RandomizedSet::new();
    assert_eq!(set.insert(1), true);
    assert_eq!(set.remove(2), false);
    assert_eq!(set.insert(2), true);
    // getRandom should return either 1 or 2
    let rand_val = set.get_random();
    assert!(rand_val == 1 || rand_val == 2);
    assert_eq!(set.remove(1), true);
    assert_eq!(set.insert(2), false);
    // Now getRandom should always return 2
    assert_eq!(set.get_random(), 2);
}
