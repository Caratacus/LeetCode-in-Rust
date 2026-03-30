// Problem 1600: throne inheritance

pub struct ThroneInheritance {
    king_name: String,
}

impl ThroneInheritance {
    pub fn new(king_name: String) -> Self {
        todo!()
    }

    pub fn birth(&mut self, parent_name: String, child_name: String) -> () {
        todo!()
    }

    pub fn death(&mut self, name: String) -> () {
        todo!()
    }

    pub fn get_inheritance_order(&self) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void throneInheritance()
    //   // order: king
    //   ThroneInheritance t = new ThroneInheritance("king");
    //   // order: king > andy
    //   t.birth("king", "andy");
    //   // order: king > andy > bob
    //   ... (22 more lines)
    #[test]
    fn test_throne_inheritance() {
        // TODO: 翻译 Java 测试
    }
}
