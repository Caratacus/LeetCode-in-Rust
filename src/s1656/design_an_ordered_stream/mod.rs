// Problem 1656: design an ordered stream

pub struct OrderedStream {
    n: i32,
}

impl OrderedStream {
    pub fn new(n: i32) -> Self {
        todo!()
    }

    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void orderedStream()
    //   OrderedStream os = new OrderedStream(5);
    //   // Inserts (3, "ccccc"), returns [].
    //   assertThat(os.insert(3, "ccccc"), equalTo(Collections.emptyList()));
    //   // Inserts (1, "aaaaa"), returns ["aaaaa"].
    //   assertThat(os.insert(1, "aaaaa"), equalTo(Arrays.asList("aaaaa")));
    //   ... (6 more lines)
    #[test]
    fn test_ordered_stream() {
        // TODO: 翻译 Java 测试
    }
}
