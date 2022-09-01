use near_sdk::{log, near_bindgen};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Debug)]
pub struct Hello {
    name: String,
}

#[near_bindgen]
impl Hello {
    // NOTE #[init] is required, otherwise it will fail to compile, or the method `new` is not callable from command line
    #[init]
    pub fn new() -> Self {
        Self {
            name: "rust".to_string()
        }
    }

    pub fn hello(&self) -> String {
        return format!("Hello, {}!", self.name.as_str()).to_string();
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = String::from(name);
        log!("Set name to {}", name);
    }
}