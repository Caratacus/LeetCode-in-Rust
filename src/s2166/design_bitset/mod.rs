// Problem 2166: design bitset

pub struct Bitset {
    size: i32,
}

impl Bitset {
    pub fn new(size: i32) -> Self {
        todo!()
    }

    pub fn fix(&mut self, idx: i32) -> () {
        todo!()
    }

    pub fn unfix(&mut self, idx: i32) -> () {
        todo!()
    }

    pub fn flip(&mut self) -> () {
        todo!()
    }

    pub fn all(&mut self) -> bool {
        todo!()
    }

    pub fn one(&mut self) -> bool {
        todo!()
    }

    pub fn count(&self) -> i32 {
        todo!()
    }

    pub fn to_string(&mut self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void bitset()
    //   // bitset = "00000".
    //   Bitset bs = new Bitset(5);
    //   // the value at idx = 3 is updated to 1, so bitset = "00010".
    //   bs.fix(3);
    //   // the value at idx = 1 is updated to 1, so bitset = "01010".
    //   ... (17 more lines)
    #[test]
    fn test_bitset() {
        // TODO: 翻译 Java 测试
    }
}
