struct Solution;

// https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&id=top-interview-150
// 3,0,6,1,5
// 0,1,3,5,6

// 0,1
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim_end().chars().rev().take_while(|c| c.is_alphabetic()).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solution58::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word(String::from("Hello World")), 5);
        assert_eq!(Solution::length_of_last_word(String::from("   fly me   to   the moon  ")), 4);
        assert_eq!(Solution::length_of_last_word(String::from("luffy is still joyboy")), 6);
    }
}