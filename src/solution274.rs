struct Solution;

// https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&id=top-interview-150
// 3,0,6,1,5
// 0,1,3,5,6

// 0,1
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let mut result = 0;
        for (i, &e) in citations.iter().rev().enumerate() {
            if e < (i + 1) as i32 {
                break;
            }
            result = result + 1;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::solution274::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
        assert_eq!(Solution::h_index(vec![0, 1]), 1);
    }
}
