impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let neg = x < 0;
        let mut ans: String = x.abs().to_string().chars().rev().collect();
        
        let a = match ans.parse::<i32>() {
            Ok(n) => if neg {-n} else {n},
            Err(_) => 0
        };

        a
    }
}