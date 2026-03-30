// Problem 0745: prefix and suffix search

pub struct WordFilter {
    words: Vec<String>,
}

impl WordFilter {
    pub fn new(words: Vec<String>) -> Self {
        todo!()
    }

    pub fn insert(&mut self, wd: String, weight: i32) -> () {
        todo!()
    }

    pub fn f(&mut self, prefix: String, suffix: String) -> i32 {
        todo!()
    }

    pub fn starts_with(&mut self, pref: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void wordFilterTest()
    //   WordFilter wordFilter = new WordFilter(new String[] {"apple"});
    //   assertThat(wordFilter.f("a", "e"), equalTo(0));
    #[test]
    fn test_word_filter_test() {
        // TODO: 翻译 Java 测试
    }
}
