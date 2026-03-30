// Problem 1032: stream of characters

pub struct StreamChecker {
    words: Vec<String>,
}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        todo!()
    }

    pub fn insert(&mut self, s: String) -> () {
        todo!()
    }

    pub fn query(&mut self, letter: char) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void streamChecker()
    //   StreamChecker streamChecker = new StreamChecker(new String[] {"cd", "f", "kl"});
    //   assertThat(streamChecker.query('a'), equalTo(false));
    //   assertThat(streamChecker.query('b'), equalTo(false));
    //   assertThat(streamChecker.query('c'), equalTo(false));
    //   assertThat(streamChecker.query('d'), equalTo(true));
    //   ... (8 more lines)
    #[test]
    fn test_stream_checker() {
        // TODO: 翻译 Java 测试
    }
}
