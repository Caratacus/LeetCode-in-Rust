// Problem 2080: range frequency queries

pub struct RangeFreqQuery {
    arr: Vec<i32>,
}

impl RangeFreqQuery {
    pub fn new(arr: Vec<i32>) -> Self {
        todo!()
    }

    pub fn query(&mut self, left: i32, right: i32, value: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void rangeFreqQuery()
    //   RangeFreqQuery rangeFreqQuery =
    //   new RangeFreqQuery(new int[] {12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56});
    //   // return 1. The value 4 occurs 1 time in the subarray [33, 4]
    //   assertThat(rangeFreqQuery.query(1, 2, 4), equalTo(1));
    //   // return 2. The value 33 occurs 2 times in the whole array.
    //   ... (1 more lines)
    #[test]
    fn test_range_freq_query() {
        // TODO: 翻译 Java 测试
    }
}
