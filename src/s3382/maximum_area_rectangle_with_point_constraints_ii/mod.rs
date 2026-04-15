// Problem 3382: maximum area rectangle with point constraints ii
// #Hard #Array #Math #Sorting #Geometry #Segment_Tree #Binary_Indexed_Tree

use std::collections::{BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        if x_coord.len() < 4 {
            return -1;
        }
        let mut pairs: Vec<(i32, i32)> = x_coord.iter().zip(y_coord.iter()).map(|(&x, &y)| (x, y)).collect();
        pairs.sort();
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut y_vals: BTreeSet<i32> = BTreeSet::new();
        let mut best: i64 = -1;
        for i in 0..pairs.len() - 1 {
            if !y_vals.is_empty() {
                let y0 = pairs[i].1;
                let mut to_check: Vec<i32> = y_vals.range(..=y0).rev().copied().collect();
                for y1 in to_check {
                    if let Some(&p1) = map.get(&y1) {
                        if p1.1 < y0 {
                            break;
                        }
                        if y1 == y0 && pairs[i + 1].0 == pairs[i].0 && pairs[i + 1].1 == p1.1 {
                            let dy = p1.1 as i64 - y0 as i64;
                            let dx = pairs[i].0 as i64 - p1.0 as i64;
                            best = best.max(dy * dx);
                        }
                        if p1.0 != pairs[i].0 {
                            y_vals.remove(&y1);
                        }
                    }
                }
            }
            if pairs[i].0 == pairs[i + 1].0 {
                y_vals.insert(pairs[i].1);
                map.insert(pairs[i].1, pairs[i + 1]);
            }
        }
        best
    }

    pub fn compare_to(p: (i32, i32)) -> i32 {
        p.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maxRectangleArea()
    //   assertThat(
    //   new Solution().maxRectangleArea(new int[] {1, 1, 3, 3}, new int[] {1, 3, 1, 3}),
    //   equalTo(4L));
    #[test]
    fn test_max_rectangle_area() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxRectangleArea2()
    //   assertThat(
    //   new Solution()
    //   .maxRectangleArea(new int[] {1, 1, 3, 3, 2}, new int[] {1, 3, 1, 3, 2}),
    //   equalTo(-1L));
    #[test]
    fn test_max_rectangle_area2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void maxRectangleArea3()
    //   assertThat(
    //   new Solution()
    //   .maxRectangleArea(
    //   new int[] {1, 1, 3, 3, 1, 3}, new int[] {1, 3, 1, 3, 2, 2}),
    //   equalTo(2L));
    #[test]
    fn test_max_rectangle_area3() {
        // TODO: 翻译 Java 测试
    }
}
