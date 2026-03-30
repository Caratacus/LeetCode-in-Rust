// Problem 0736: parse lisp expression

pub struct Solution;

impl Solution {
    pub fn evaluate(exp: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void evaluate()
    //   assertThat(
    //   new Solution().evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))"), equalTo(14));
    #[test]
    fn test_evaluate() {
        // TODO: 翻译 Java 测试
    }

    // Java: void evaluate2()
    //   assertThat(new Solution().evaluate("(let x 3 x 2 x)"), equalTo(2));
    #[test]
    fn test_evaluate2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void evaluate3()
    //   assertThat(new Solution().evaluate("(let x 1 y 2 x (add x y) (add x y))"), equalTo(5));
    #[test]
    fn test_evaluate3() {
        // TODO: 翻译 Java 测试
    }
}
