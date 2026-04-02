// Tests for Problem 0146: LRU Cache
// Java reference: src/test/java/g0121_0200/s0146_lru_cache/SolutionTest.java

use leetcode_in_rust::s0146::lru_cache::LRUCache;

#[test]
fn test_lru_cache() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3); // evicts key 2
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4); // evicts key 1
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
