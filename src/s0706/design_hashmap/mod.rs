// Problem 0706: design hashmap

pub struct MyHashMap {}

impl MyHashMap {
    pub fn new() -> Self {
        todo!()
    }

    pub fn put(&mut self, key: i32, value: i32) -> () {
        todo!()
    }

    pub fn get(&self, key: i32) -> i32 {
        todo!()
    }

    pub fn remove(&mut self, key: i32) -> () {
        todo!()
    }

    pub fn hash_code(&mut self) -> i32 {
        todo!()
    }

    pub fn equals(&mut self, obj: Box<dyn std::any::Any>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void myHashMapTest()
    //   MyHashMap myHashMap = new MyHashMap();
    //   myHashMap.put(1, 1);
    //   myHashMap.put(2, 2);
    //   assertThat(myHashMap.get(1), equalTo(1));
    //   assertThat(myHashMap.get(3), equalTo(-1));
    //   ... (4 more lines)
    #[test]
    fn test_my_hash_map_test() {
        // TODO: 翻译 Java 测试
    }
}
