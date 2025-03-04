impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let (mut even_sum, mut odd_sum) = (0, 0);

        for (i, ch) in num.chars().enumerate() {
            let val = ch.to_digit(10).unwrap_or(0);
            match i % 2 {
                0 => even_sum += val,
                _ => odd_sum += val,
            }
        }

        even_sum == odd_sum
    }
}