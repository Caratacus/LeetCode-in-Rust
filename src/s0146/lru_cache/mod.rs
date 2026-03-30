// Problem 0146: lru cache

pub struct LRUCache {
    cap: i32,
}

impl LRUCache {
    pub fn new(cap: i32) -> Self {
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

    // Java: void lruCache()
    //   LRUCache lruCache = new LRUCache(2);
    //   // cache is {1=1}
    //   lruCache.put(1, 1);
    //   // cache is {1=1, 2=2}
    //   lruCache.put(2, 2);
    //   ... (14 more lines)
    #[test]
    fn test_lru_cache() {
        // TODO: 翻译 Java 测试
    }
}
