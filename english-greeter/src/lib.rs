use hello_core::{Greeter, GreetingConfig, capitalize_first_letter};

pub struct EnglishGreeter {
    config: GreetingConfig,
}

impl EnglishGreeter {
    pub fn new(config: GreetingConfig) -> Self {
        Self { config }
    }
}

impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        let capitalized_name = capitalize_first_letter(name);
        
        if self.config.formal {
            format!("Good {}, Mr./Ms. {}!", self.config.time_of_day, capitalized_name)
        } else {
            format!("Hi {}, good {}!", capitalized_name, self.config.time_of_day)
        }
    }

    fn farewell(&self, name: &str) -> String {
        let capitalized_name = capitalize_first_letter(name);
        
        if self.config.formal {
            format!("Goodbye, Mr./Ms. {}!", capitalized_name)
        } else {
            format!("See you later, {}!", capitalized_name)
        }
    }
}