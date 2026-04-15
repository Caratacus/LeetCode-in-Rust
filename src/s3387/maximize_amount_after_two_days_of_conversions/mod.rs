// Problem 3387: maximize amount after two days of conversions
// #Medium #Array #String #DFS #BFS #Graph

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn max_amount(
        initial_currency: String,
        pairs1: Vec<Vec<String>>,
        rates1: Vec<f64>,
        pairs2: Vec<Vec<String>>,
        rates2: Vec<f64>,
    ) -> f64 {
        let mut map1: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        let mut map2: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        for (i, p) in pairs1.iter().enumerate() {
            let c1 = &p[0];
            let c2 = &p[1];
            map1.entry(c1.as_str()).or_default().push((c2.as_str(), rates1[i]));
            map1.entry(c2.as_str()).or_default().push((c1.as_str(), 1.0 / rates1[i]));
        }
        for (i, p) in pairs2.iter().enumerate() {
            let c1 = &p[0];
            let c2 = &p[1];
            map2.entry(c1.as_str()).or_default().push((c2.as_str(), rates2[i]));
            map2.entry(c2.as_str()).or_default().push((c1.as_str(), 1.0 / rates2[i]));
        }
        let mut res = 1.0f64;
        Self::solve(&initial_currency, 1.0, &initial_currency, 1, &mut HashSet::new(), &map1, &map2, &mut res);
        res
    }

    fn solve<'a>(
        curr: &str,
        value: f64,
        target: &str,
        day: i32,
        used: &mut HashSet<&'a str>,
        map1: &HashMap<&'a str, Vec<(&'a str, f64)>>,
        map2: &HashMap<&'a str, Vec<(&'a str, f64)>>,
        res: &mut f64,
    ) {
        if curr == target {
            *res = res.max(value);
            if day == 2 {
                return;
            }
        }
        let list = if day == 1 {
            map1.get(curr).cloned().unwrap_or_default()
        } else {
            map2.get(curr).cloned().unwrap_or_default()
        };
        for (next, rate) in &list {
            if used.insert(next) {
                Self::solve(next, value * rate, target, day, used, map1, map2, res);
                used.remove(next);
            }
        }
        if day == 1 {
            Self::solve(curr, value, target, day + 1, &mut HashSet::new(), map1, map2, res);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxAmount()
    //   assertThat(
    //   new Solution()
    //   .maxAmount(
    //   "EUR",
    //   List.of(List.of("EUR", "USD"), List.of("USD", "JPY")),
    //   ... (7 more lines)
    #[test]
    fn test_max_amount() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxAmount2()
    //   assertThat(
    //   new Solution()
    //   .maxAmount(
    //   "NGN",
    //   List.of(List.of("NGN", "EUR")),
    //   ... (4 more lines)
    #[test]
    fn test_max_amount2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxAmount3()
    //   assertThat(
    //   new Solution()
    //   .maxAmount(
    //   "USD",
    //   List.of(List.of("USD", "EUR")),
    //   ... (4 more lines)
    #[test]
    fn test_max_amount3() {
        // TODO: 翻译 Java 测试
    }
}
