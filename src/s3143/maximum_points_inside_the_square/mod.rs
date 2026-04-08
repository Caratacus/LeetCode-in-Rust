// Problem 3143: maximum points inside the square
// #Medium #Array #String #Hash_Table #Sorting #Binary_Search
// #2024_05_15_Time_2_ms_(100.00%)_Space_100.1_MB_(61.27%)

pub struct Solution;

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut tags = [i32::MAX; 26];
        let mut second_min = i32::MAX;
        let s_bytes = s.as_bytes();
        for i in 0..s.len() {
            let dist = points[i][0].abs().max(points[i][1].abs());
            let c = (s_bytes[i] - b'a') as usize;
            if tags[c] == i32::MAX {
                tags[c] = dist;
            } else if dist < tags[c] {
                second_min = second_min.min(tags[c]);
                tags[c] = dist;
            } else {
                second_min = second_min.min(dist);
            }
        }
        let mut count = 0;
        for &dist in tags.iter() {
            if dist < second_min {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_points_inside_square() {
        assert_eq!(
            Solution::max_points_inside_square(
                vec![vec![2, 2], vec![-1, -2], vec![-4, 4], vec![-3, 1], vec![3, -3]],
                "abdca".to_string()
            ),
            2
        );
    }

    #[test]
    fn test_max_points_inside_square2() {
        assert_eq!(
            Solution::max_points_inside_square(
                vec![vec![1, 1], vec![-2, -2], vec![-2, 2]],
                "abb".to_string()
            ),
            1
        );
    }
}
