// Problem 1286: iterator for combination

pub struct CombinationIterator {
    characters: String,
    combination_length: i32,
}

impl CombinationIterator {
    pub fn new(characters: String, combination_length: i32) -> Self {
        todo!()
    }

    pub fn next(&mut self) -> String {
        todo!()
    }

    pub fn has_next(&mut self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void combinationIterator()
    //   CombinationIterator itr = new CombinationIterator("abc", 2);
    //   assertThat(itr.next(), equalTo("ab"));
    //   assertThat(itr.hasNext(), equalTo(true));
    //   assertThat(itr.next(), equalTo("ac"));
    //   assertThat(itr.hasNext(), equalTo(true));
    //   ... (2 more lines)
    #[test]
    fn test_combination_iterator() {
        // TODO: 翻译 Java 测试
    }
}
