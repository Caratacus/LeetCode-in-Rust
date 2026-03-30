// Problem 0770: basic calculator iv

pub struct Solution;

impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void basicCalculatorIV()
    //   assertThat(
    //   new Solution()
    //   .basicCalculatorIV("e + 8 - a + 5", new String[] {"e"}, new int[] {1}),
    //   equalTo(Arrays.asList("-1*a", "14")));
    #[test]
    fn test_basic_calculator_iv() {
        // TODO: 翻译 Java 测试
    }

    // Java: void basicCalculatorIV2()
    //   assertThat(
    //   new Solution()
    //   .basicCalculatorIV(
    //   "e - 8 + temperature - pressure",
    //   new String[] {"e", "temperature"},
    //   ... (2 more lines)
    #[test]
    fn test_basic_calculator_iv2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void basicCalculatorIV3()
    //   assertThat(
    //   new Solution()
    //   .basicCalculatorIV("(e + 8) * (e - 8)", new String[] {}, new int[] {}),
    //   equalTo(Arrays.asList("1*e*e", "-64")));
    #[test]
    fn test_basic_calculator_iv3() {
        // TODO: 翻译 Java 测试
    }
}
