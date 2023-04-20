pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup, // 枚举成员无需设置pub，只需要在枚举上设置 pub，所有成员都公开了
    Salad,
}