// Tests for Problem 0460: LFU Cache
// Java reference: src/test/java/g0401_0500/s0460_lfu_cache/SolutionTest.java

use leetcode_in_rust::s0460::lfu_cache::LFUCache;

#[test]
fn test_lfu_cache() {
    let mut cache = LFUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3); // evicts key 2
    assert_eq!(cache.get(2), -1);
    assert_eq!(cache.get(3), 3);
    cache.put(4, 4); // evicts key 1
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}
