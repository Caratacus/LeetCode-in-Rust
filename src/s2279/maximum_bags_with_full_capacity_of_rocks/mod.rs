// Problem 2279: maximum bags with full capacity of rocks

pub struct Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let n = capacity.len();
        let mut remaining: Vec<i64> = (0..n)
            .map(|i| capacity[i] as i64 - rocks[i] as i64)
            .collect();
        remaining.sort();
        let mut additional = additional_rocks as i64;
        let mut count = 0;
        for r in remaining {
            if r == 0 {
                count += 1;
            } else if additional >= r {
                additional -= r;
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumBags()
    //   assertThat(
    //   new Solution().maximumBags(new int[] {2, 3, 4, 5}, new int[] {1, 2, 4, 4}, 2),
    //   equalTo(3));
    #[test]
    fn test_maximum_bags() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maximumBags2()
    //   assertThat(
    //   new Solution().maximumBags(new int[] {10, 2, 2}, new int[] {2, 2, 0}, 100),
    //   equalTo(3));
    #[test]
    fn test_maximum_bags2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maximumBags3()
    //   assertThat(
    //   new Solution()
    //   .maximumBags(
    //   new int[] {91, 54, 63, 99, 24, 45, 78},
    //   new int[] {35, 32, 45, 98, 6, 1, 25},
    //   ... (2 more lines)
    #[test]
    fn test_maximum_bags3() {
        // TODO: 翻译 Java 测试
    }
}
