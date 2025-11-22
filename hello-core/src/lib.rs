// 定義共用的特徵
pub trait Greeter {
    fn greet(&self, name: &str) -> String;
    fn farewell(&self, name: &str) -> String;
}

// 共用的配置結構
#[derive(Debug)]
pub struct GreetingConfig {
    pub formal: bool,
    pub time_of_day: String, // "morning", "afternoon", "evening"
}

// 提供一些工具函數
pub fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
