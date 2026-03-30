// Problem 0900: rle iterator

pub struct RLEIterator {
    encoding: Vec<i32>,
}

impl RLEIterator {
    pub fn new(encoding: Vec<i32>) -> Self {
        todo!()
    }

    pub fn next(&mut self, n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rleIteratorTest()
    //   RLEIterator rleIterator = new RLEIterator(new int[] {3, 8, 0, 9, 2, 5});
    //   assertThat(rleIterator.next(2), equalTo(8));
    //   assertThat(rleIterator.next(1), equalTo(8));
    //   assertThat(rleIterator.next(1), equalTo(5));
    //   assertThat(rleIterator.next(2), equalTo(-1));
    #[test]
    fn test_rle_iterator_test() {
        // TODO: 翻译 Java 测试
    }
}
