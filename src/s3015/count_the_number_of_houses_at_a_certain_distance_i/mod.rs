// Problem 3015: count the number of houses at a certain distance i
// #Medium #Breadth_First_Search #Graph #Prefix_Sum

pub struct Solution;

impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let n = n as usize;
        let mut answer = vec![0i32; n];
        let mut distance = n - 1;
        let mut k = distance - 1;
        while distance > 0 {
            answer[k] = ((n - distance) * 2) as i32;
            distance -= 1;
            if k > 0 {
                k -= 1;
            }
        }

        let (mut x, mut y) = (x, y);
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }

        let skip = y - x;
        if skip < 2 {
            return answer;
        }

        for i in 1..=n as i32 {
            for j in (i + 1)..=n as i32 {
                let old_distance = (j - i) as usize;
                let new_distance = ((x - i).abs() + (y - j).abs() + 1) as usize;
                if new_distance < old_distance {
                    answer[old_distance - 1] -= 2;
                    answer[new_distance - 1] += 2;
                }
            }
        }

        answer
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
