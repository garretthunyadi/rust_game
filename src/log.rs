use crate::s;
use std::cell::RefCell;
use std::fmt;

pub struct Log {
    messages: RefCell<Vec<String>>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            messages: RefCell::new(vec![]),
        }
    }
    pub fn log(&self, message: &str) {
        self.messages.borrow_mut().push(s!(message));
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LOG : {}", self.messages.borrow().join("\nLOG : "))
    }
}
