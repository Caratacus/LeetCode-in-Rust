// Problem 0460: lfu cache

pub struct LFUCache {
    capacity: i32,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        todo!()
    }

    pub fn get(&self, key: i32) -> i32 {
        todo!()
    }

    pub fn put(&mut self, key: i32, value: i32) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void lfuCache()
    //   LFUCache lfuCache = new LFUCache(2);
    //   lfuCache.put(1, 1);
    //   lfuCache.put(2, 2);
    //   assertThat(lfuCache.get(1), equalTo(1));
    //   lfuCache.put(3, 3);
    //   ... (6 more lines)
    #[test]
    fn test_lfu_cache() {
        // TODO: 翻译 Java 测试
    }
}
