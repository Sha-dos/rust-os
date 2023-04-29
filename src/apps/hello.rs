use alloc::string::{String, ToString};
use crate::apps::App;
use crate::println;

pub struct HelloApp;

impl HelloApp {
    pub fn new() -> Self {
        Self
    }
}

impl App for HelloApp {
    fn run(&self) {
        println!("Hello From An App!")
    }

    fn name(&self) -> String {
        "hello_app".to_string()
    }
}