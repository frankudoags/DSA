impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let (mut even_sum, mut odd_sum) = (0, 0);

        for (i, ch) in num.into_bytes().into_iter().enumerate() {
            let val = ch - b'0';
            match i % 2 {
                0 => even_sum += val,
                _ => odd_sum += val,
            }
        }

        even_sum == odd_sum
    }
}