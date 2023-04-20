mod back_of_house; // 此处只定义模块，模块的内容在 back_of_house.rs 中，rust 会自动搜索该文件名
mod front_of_house;

mod order {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 通过 supper 从根开始查找函数
        // super::front_of_house::hosting::seat_at_table() // 不能访问其他模块中的私有函数
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // crate::front_of_house::hosting::seat_at_table(); // 不能访问子模块中的私有函数

    use crate::front_of_house::hosting; // 使用 use 关键字将模块引入作用域，引入后就可以直接使用模块名前缀调用函数了
    hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    // front_of_house::serving::take_order(); // 不能访问私有子模块中的函数

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat"); // toast 是公开的，所以可以直接修改
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // seasonal_fruit 是私有的，不能直接修改

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

// pub use 从根模块重导出了 hosting 模块，外部代码现在可以使用路径 restaurant::hosting::add_to_waitlist，在这个修改之前，外部代码需要使用路径 restaurant::front_of_house::hosting::add_to_waitlist() 来调用 add_to_waitlist 函数
pub use crate::front_of_house::hosting;