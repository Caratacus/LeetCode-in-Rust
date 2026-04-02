// Tests for Problem 0381: Insert Delete GetRandom O(1) - Duplicates allowed
// Java reference: src/test/java/g0301_0400/s0381_insert_delete_getrandom_o1_duplicates_allowed/RandomizedCollectionTest.java

use leetcode_in_rust::s0381::insert_delete_getrandom_o1_duplicates_allowed::RandomizedCollection;

#[test]
fn test_randomized_collection() {
    let mut collection = RandomizedCollection::new();
    assert_eq!(collection.insert(1), true);
    assert_eq!(collection.insert(1), false);
    assert_eq!(collection.insert(2), true);
    // getRandom should return either 1 or 2
    let rand_val = collection.get_random();
    assert!(rand_val == 1 || rand_val == 2);
    assert_eq!(collection.remove(1), true);
    // After removing one 1, getRandom should always return 2 (since one 1 remains)
    let rand_val2 = collection.get_random();
    assert!(rand_val2 == 1 || rand_val2 == 2);
}
