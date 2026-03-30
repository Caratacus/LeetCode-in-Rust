// Problem 1146: snapshot array

pub struct SnapshotArray {
    length: i32,
}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        todo!()
    }

    pub fn set(&mut self, index: i32, val: i32) -> () {
        todo!()
    }

    pub fn snap(&mut self) -> i32 {
        todo!()
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void snapshotArrayTest()
    //   SnapshotArray snapshotArr = new SnapshotArray(3);
    //   snapshotArr.set(0, 5);
    //   assertThat(snapshotArr.snap(), equalTo(0));
    //   snapshotArr.set(0, 6);
    //   assertThat(snapshotArr.get(0, 0), equalTo(5));
    #[test]
    fn test_snapshot_array_test() {
        // TODO: 翻译 Java 测试
    }
}
