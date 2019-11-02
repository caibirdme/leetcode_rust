use std::collections::HashMap;

struct Logger {
    hash: HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self{
            hash: HashMap::new(),
        }
    }

    /** Returns true if the message should be printed in the given timestamp, otherwise returns false.
        If this method returns false, the message will not be printed.
        The timestamp is in seconds granularity. */
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if !self.hash.contains_key(&message) {
            self.hash.insert(message, timestamp);
            return true;
        }
        let p = self.hash.get_mut(&message).unwrap();
        if timestamp - *p >= 10 {
            *p = timestamp;
            return true;
        }
        false
    }
}