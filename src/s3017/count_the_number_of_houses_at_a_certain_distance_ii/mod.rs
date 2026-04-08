// Problem 3017: count the number of houses at a certain distance ii
// #Hard #Breadth_First_Search #Graph #Prefix_Sum

pub struct Solution;

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i64> {
        let n = n as usize;
        let mut result = vec![0i64; n];

        let left_count = (x.min(y) - 1) as i32;
        let right_count = (n as i32 - x.max(y)) as i32;
        let circle_count = (n as i32 - left_count - right_count) as i32;

        Self::circle_internal(circle_count, &mut result);
        Self::line_to_circle(left_count, circle_count, &mut result);
        Self::line_to_circle(right_count, circle_count, &mut result);
        Self::line_to_line(
            left_count,
            right_count,
            if x == y { 1 } else { 2 },
            &mut result,
        );
        Self::line_internal(left_count, &mut result);
        Self::line_internal(right_count, &mut result);

        result
    }

    fn line_to_circle(line_count: i32, circle_count: i32, cur_res: &mut [i64]) {
        let circle_len = circle_count / 2 + 1;
        let mut cur_modifier = 0;

        for i in 1..=(circle_len + line_count - 1) {
            if i <= line_count.min(circle_len) {
                cur_modifier += 4;
            } else if i > line_count.max(circle_len) {
                cur_modifier -= 4;
            }
            cur_res[(i - 1) as usize] += cur_modifier;

            if i <= line_count {
                cur_res[(i - 1) as usize] -= 2;
            }
            if i >= circle_len && circle_count % 2 == 0 {
                cur_res[(i - 1) as usize] -= 2;
            }
        }
    }

    fn line_to_line(line_count1: i32, line_count2: i32, initial_dis: i32, cur_res: &mut [i64]) {
        let mut cur_modifier = 0;

        for i in 1..=(line_count1 + line_count2 - 1) {
            if i <= line_count1.min(line_count2) {
                cur_modifier += 2;
            } else if i > line_count1.max(line_count2) {
                cur_modifier -= 2;
            }
            cur_res[(i - 1 + initial_dis) as usize] += cur_modifier;
        }
    }

    fn line_internal(line_count: i32, cur_res: &mut [i64]) {
        for i in 1..line_count {
            cur_res[(i - 1) as usize] += (line_count - i) as i64 * 2;
        }
    }

    fn circle_internal(circle_count: i32, cur_res: &mut [i64]) {
        for i in 0..(circle_count / 2) {
            if circle_count % 2 == 0 && i + 1 == circle_count / 2 {
                cur_res[i as usize] += circle_count as i64;
            } else {
                cur_res[i as usize] += circle_count as i64 * 2;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_of_pairs() {
        assert_eq!(Solution::count_of_pairs(3, 1, 3), vec![6, 0, 0]);
    }

    #[test]
    fn test_count_of_pairs2() {
        assert_eq!(Solution::count_of_pairs(5, 2, 4), vec![10, 8, 2, 0, 0]);
    }

    #[test]
    fn test_count_of_pairs3() {
        assert_eq!(Solution::count_of_pairs(4, 1, 1), vec![6, 4, 2, 0]);
    }
}
