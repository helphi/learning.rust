// 将模块设为 pub 以便外部可以使用
// 模块的内容在 front_of_house/hosting.rs 中，rust 会自动搜索该文件路径
pub mod hosting; 

mod serving { // 私有模块，外部不能使用
    pub fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}