use hello_core::{Greeter, GreetingConfig};

pub struct ChineseGreeter {
    config: GreetingConfig,
}

impl ChineseGreeter {
    pub fn new(config: GreetingConfig) -> Self {
        Self { config }
    }
}

impl Greeter for ChineseGreeter {
    fn greet(&self, name: &str) -> String {
        let time_greeting = match self.config.time_of_day.as_str() {
            "morning" => "早安",
            "afternoon" => "午安", 
            "evening" => "晚安",
            _ => "你好",
        };

        if self.config.formal {
            format!("{}，{}先生/女士！", time_greeting, name)
        } else {
            format!("{}，{}！", time_greeting, name)
        }
    }

    fn farewell(&self, name: &str) -> String {
        if self.config.formal {
            format!("再見，{}先生/女士！", name)
        } else {
            format!("再見，{}！", name)
        }
    }
}