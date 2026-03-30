// Problem 0284: peeking iterator

pub struct PeekingIterator {
    iterator: Box<dyn Iterator<Item = i32>>,
    peeked: Option<i32>,
}

impl PeekingIterator {
    pub fn new(iterator: Box<dyn Iterator<Item = i32>>) -> Self {
        todo!()
    }

    pub fn peek(&self) -> i32 {
        todo!()
    }

    pub fn next(&mut self) -> i32 {
        todo!()
    }

    pub fn has_next(&mut self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void peekingIterator()
    //   // [1,2,3]
    //   PeekingIterator peekingIterator = new PeekingIterator(Arrays.asList(1, 2, 3).iterator());
    //   assertThat(peekingIterator.next(), equalTo(1));
    //   assertThat(peekingIterator.peek(), equalTo(2));
    //   assertThat(peekingIterator.next(), equalTo(2));
    //   assertThat(peekingIterator.next(), equalTo(3));
    //   ... (7 more lines)
    #[test]
    fn test_peeking_iterator() {
        // TODO: 翻译 Java 测试
    }
}
