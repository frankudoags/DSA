use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        while self.queue.front().map_or(false, |x| *x < t - 3000) {
            self.queue.pop_front();
        }

        self.queue.push_back(t);
        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */