// Problem 1410: html entity parser

pub struct Solution;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void entityParser()
    //   assertThat(
    //   new Solution().entityParser("&amp; is an HTML entity but &ambassador; is not."),
    //   equalTo("& is an HTML entity but &ambassador; is not."));
    #[test]
    fn test_entity_parser() {
        // TODO: 翻译 Java 测试
    }

    // Java: void entityParser2()
    //   assertThat(
    //   new Solution().entityParser("and I quote: &quot;...&quot;"),
    //   equalTo("and I quote: \"...\""));
    #[test]
    fn test_entity_parser2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void entityParser3()
    //   assertThat(new Solution().entityParser("&frasl;&apos;&gt;&lt;&lt"), equalTo("/'><&lt"));
    #[test]
    fn test_entity_parser3() {
        // TODO: 翻译 Java 测试
    }
}
