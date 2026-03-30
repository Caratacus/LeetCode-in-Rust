// Problem 0981: time based key value store

pub struct TimeMap {}

impl TimeMap {
    pub fn new() -> Self {
        todo!()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) -> () {
        todo!()
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void timeMap()
    //   TimeMap timeMap = new TimeMap();
    //   timeMap.set("foo", "bar", 1);
    //   assertThat(timeMap.get("foo", 1), equalTo("bar"));
    //   assertThat(timeMap.get("foo", 3), equalTo("bar"));
    //   timeMap.set("foo", "bar2", 4);
    //   ... (2 more lines)
    #[test]
    fn test_time_map() {
        // TODO: 翻译 Java 测试
    }
}
