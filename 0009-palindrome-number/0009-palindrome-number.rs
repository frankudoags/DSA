impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {return false }

        return x == x.to_string().chars().rev().collect::<String>().parse::<i128>().unwrap() as i32
    }
}