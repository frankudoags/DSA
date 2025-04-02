impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let (mut max_i, mut max_diff, mut res): (i64, i64, i64) = (0, 0, 0);

        for (k, n) in nums.iter().enumerate() {
            let num = *n as i64; 
            res = res.max(max_diff * num);
            max_diff = max_diff.max(max_i - num);
            max_i = max_i.max(num);
        }

        res.into()
    }
}