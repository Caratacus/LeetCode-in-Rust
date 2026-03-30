// Problem 2502: design memory allocator

pub struct Allocator {
    n: i32,
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        todo!()
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        todo!()
    }

    pub fn free(&mut self, m_id: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void allocator()
    //   // Initialize a memory array of size 10. All memory units are initially free.
    //   Allocator loc = new Allocator(10);
    //   // The leftmost block's first index is 0. The memory array becomes [1,_,_,_,_,_,_,_,_,_]. We
    //   // return 0.
    //   assertThat(loc.allocate(1, 1), equalTo(0));
    //   ... (26 more lines)
    #[test]
    fn test_allocator() {
        // TODO: 翻译 Java 测试
    }
}
