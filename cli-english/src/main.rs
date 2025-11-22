use hello_core::Greeter;
use hello_core::GreetingConfig;
use english_greeter::EnglishGreeter;

fn main() {
    println!("=== 英文問候示範 ===");
    
    // 正式問候
    let formal_config = GreetingConfig {
        formal: true,
        time_of_day: "morning".to_string(),
    };
    
    let formal_greeter = EnglishGreeter::new(formal_config);
    println!("正式: {}", formal_greeter.greet("alice"));
    println!("告別: {}", formal_greeter.farewell("alice"));
    
    println!();
    
    // 非正式問候
    let informal_config = GreetingConfig {
        formal: false,
        time_of_day: "evening".to_string(),
    };
    
    let informal_greeter = EnglishGreeter::new(informal_config);
    println!("非正式: {}", informal_greeter.greet("bob"));
    println!("告別: {}", informal_greeter.farewell("bob"));
}