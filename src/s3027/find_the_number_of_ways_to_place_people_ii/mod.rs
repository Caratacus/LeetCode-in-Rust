// Problem 3027: Find the Number of Ways to Place People II
// #Hard #Array #Math #Sorting #Enumeration #Geometry

pub struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();

        // Sort by x ascending, then by y descending
        points.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        let mut count = 0;
        for i in 0..points.len() {
            let mut m = i32::MIN;
            for j in (i + 1)..points.len() {
                if points[i].1 >= points[j].1 && points[j].1 > m {
                    m = points[j].1;
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void numberOfPairs()
    //   assertThat(new Solution().numberOfPairs(new int[][] {{1, 1}, {2, 2}, {3, 3}}), equalTo(0));
    #[test]
    fn test_number_of_pairs() {
        assert_eq!(
            Solution::number_of_pairs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            0
        );
    }

    // Java: void numberOfPairs2()
    //   assertThat(new Solution().numberOfPairs(new int[][] {{6, 2}, {4, 4}, {2, 6}}), equalTo(2));
    #[test]
    fn test_number_of_pairs2() {
        assert_eq!(
            Solution::number_of_pairs(vec![vec![6, 2], vec![4, 4], vec![2, 6]]),
            2
        );
    }

    // Java: void numberOfPairs3()
    //   assertThat(new Solution().numberOfPairs(new int[][] {{3, 1}, {1, 3}, {1, 1}}), equalTo(2));
    #[test]
    fn test_number_of_pairs3() {
        assert_eq!(
            Solution::number_of_pairs(vec![vec![3, 1], vec![1, 3], vec![1, 1]]),
            2
        );
    }
}
