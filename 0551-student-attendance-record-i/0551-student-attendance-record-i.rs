impl Solution {
    pub fn check_record(s: String) -> bool {
        let [mut absent, mut c_late] = [0, 0];

        for status in s.chars() {
            match status {
                'A' => {
                    absent += 1;
                    c_late = 0;
                },
                'L' => c_late += 1,
                _ => c_late = 0,
            }

            if absent >= 2 || c_late >= 3 {
                return false;
            
            }

        }

        true
    }
}