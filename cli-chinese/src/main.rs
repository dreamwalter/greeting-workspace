use hello_core::Greeter;
use hello_core::GreetingConfig;
use chinese_greeter::ChineseGreeter;

fn main() {
    println!("=== 中文問候示範 ===");
    
    // 正式問候
    let formal_config = GreetingConfig {
        formal: true,
        time_of_day: "afternoon".to_string(),
    };
    
    let formal_greeter = ChineseGreeter::new(formal_config);
    println!("正式: {}", formal_greeter.greet("王小明"));
    println!("告別: {}", formal_greeter.farewell("王小明"));
    
    println!();
    
    // 非正式問候
    let informal_config = GreetingConfig {
        formal: false, 
        time_of_day: "morning".to_string(),
    };
    
    let informal_greeter = ChineseGreeter::new(informal_config);
    println!("非正式: {}", informal_greeter.greet("小李"));
    println!("告別: {}", informal_greeter.farewell("小李"));
}